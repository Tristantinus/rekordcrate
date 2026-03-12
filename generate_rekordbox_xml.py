#!/usr/bin/env python3
"""
Generate rekordbox.xml from M3U playlist files.
Reads M3U files from playlists-j, deduplicates tracks, builds playlist tree.
Reads ID3/Vorbis/MP4 tags from audio files using mutagen.
"""

import os
import xml.etree.ElementTree as ET
from urllib.parse import quote, unquote
from pathlib import Path
from collections import defaultdict

from mutagen import File as MutagenFile

M3U_ROOT   = r'C:\Users\trist\Desktop\playlists-j'
OUTPUT_XML = r'C:\Users\trist\Desktop\playlists-j\rekordbox_from_m3u.xml'

def read_m3u(path):
    with open(path, 'rb') as f:
        raw = f.read()
    if raw.startswith(b'\xef\xbb\xbf'):
        raw = raw[3:]
    return raw.decode('utf-8', errors='replace')

def path_to_location(win_path):
    p = win_path.replace('\\', '/')
    parts = p.split('/')
    encoded_parts = [parts[0]] + [quote(part, safe='') for part in parts[1:]]
    return f'file://localhost/{"/".join(encoded_parts)}'

def location_to_name(location):
    filename = location.rstrip('/').split('/')[-1]
    name = unquote(filename)
    return name.rsplit('.', 1)[0] if '.' in name else name

KIND_MAP = {
    'MP3': 'MP3 File', 'FLAC': 'FLAC File', 'M4A': 'AAC File',
    'AAC': 'AAC File', 'WAV': 'WAV File', 'AIFF': 'AIFF File',
    'AIF': 'AIFF File', 'OGG': 'OGG File', 'ALAC': 'ALAC File',
}

def read_tags(win_path):
    """Read metadata from audio file. Returns dict of rekordbox TRACK attributes."""
    basename = os.path.basename(win_path)
    ext = basename.rsplit('.', 1)[-1].upper() if '.' in basename else ''
    name_fallback = basename.rsplit('.', 1)[0] if '.' in basename else basename

    tags = {
        'Name': name_fallback, 'Artist': '', 'Composer': '', 'Album': '',
        'Grouping': '', 'Genre': '', 'Kind': KIND_MAP.get(ext, ext + ' File' if ext else 'MP3 File'),
        'Size': '0', 'TotalTime': '0', 'DiscNumber': '0', 'TrackNumber': '0',
        'Year': '', 'AverageBpm': '0.00', 'DateAdded': '2024-01-01',
        'BitRate': '0', 'SampleRate': '44100', 'Comments': '', 'PlayCount': '0',
        'Rating': '0', 'Remixer': '', 'Tonality': '', 'Label': '', 'Mix': '',
    }

    try:
        tags['Size'] = str(os.path.getsize(win_path))
    except Exception:
        pass

    try:
        f = MutagenFile(win_path, easy=True)
        if f is None:
            return tags

        # Audio info: duration, bitrate, sample rate
        if hasattr(f, 'info'):
            info = f.info
            if hasattr(info, 'length') and info.length:
                tags['TotalTime'] = str(int(info.length))
            if hasattr(info, 'bitrate') and info.bitrate:
                tags['BitRate'] = str(info.bitrate // 1000)
            if hasattr(info, 'sample_rate') and info.sample_rate:
                tags['SampleRate'] = str(info.sample_rate)

        # Easy tags (normalized across MP3/FLAC/MP4/etc.)
        et = f.tags or {}
        def eg(key):
            v = et.get(key)
            return str(v[0]) if v else ''

        t = eg('title');     tags['Name']    = t if t else tags['Name']
        a = eg('artist');    tags['Artist']  = a
        al = eg('album');    tags['Album']   = al
        g = eg('genre');     tags['Genre']   = g
        c = eg('composer');  tags['Composer']= c
        dt = eg('date');     tags['Year']    = dt[:4] if dt else ''
        tn = eg('tracknumber')
        if tn: tags['TrackNumber'] = tn.split('/')[0]
        bpm = eg('bpm')
        if bpm:
            try: tags['AverageBpm'] = f'{float(bpm):.2f}'
            except Exception: pass
        org = eg('organization')
        if org: tags['Label'] = org

    except Exception:
        pass

    # Non-easy tags: key (TKEY), remixer (TPE4), comments (COMM), label (TPUB)
    try:
        f2 = MutagenFile(win_path, easy=False)
        if f2 and f2.tags:
            t2 = f2.tags
            tkey = t2.get('TKEY')
            if tkey: tags['Tonality'] = str(tkey)
            tpe4 = t2.get('TPE4')
            if tpe4: tags['Remixer'] = str(tpe4)
            if not tags['Label']:
                tpub = t2.get('TPUB')
                if tpub: tags['Label'] = str(tpub)
            for k in list(t2.keys()):
                if k.startswith('COMM'):
                    comm = t2[k]
                    tags['Comments'] = str(comm.text[0]) if hasattr(comm, 'text') else str(comm)
                    break
    except Exception:
        pass

    return tags

# ---- Collect all tracks from all M3U files ----
track_map  = {}   # normalized location -> TrackID
track_info = {}   # TrackID -> {tags dict + Location}
next_id = 1
playlists = []

m3u_root  = Path(M3U_ROOT)
m3u_files = sorted(m3u_root.rglob('*.m3u'))
print(f"Found {len(m3u_files)} M3U files")

for m3u_path in m3u_files:
    content = read_m3u(str(m3u_path))
    track_locations = []
    for line in content.splitlines():
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        if len(line) > 3 and line[1] == ':' and line[2] in ('\\', '/'):
            location = path_to_location(line)
            norm = location.lower()
            if norm not in track_map:
                track_map[norm] = next_id
                tags = read_tags(line)
                tags['Location'] = location
                track_info[next_id] = tags
                next_id += 1
            track_locations.append(norm)

    rel = m3u_path.relative_to(m3u_root)
    parts = list(rel.parts)
    playlist_name = parts[-1].rsplit('.', 1)[0]
    folder_parts  = tuple(parts[:-1])
    playlists.append((folder_parts, playlist_name, track_locations))

print(f"Unique tracks: {len(track_info)}, Playlists: {len(playlists)}")

# ---- Build XML ----
root = ET.Element('DJ_PLAYLISTS', Version='1.0.0')
ET.SubElement(root, 'PRODUCT', Name='rekordbox', Version='6.0.0', Company='AlphaTheta')

collection = ET.SubElement(root, 'COLLECTION', Entries=str(len(track_info)))

TRACK_ATTR_ORDER = [
    'TrackID', 'Name', 'Artist', 'Composer', 'Album', 'Grouping', 'Genre',
    'Kind', 'Size', 'TotalTime', 'DiscNumber', 'TrackNumber', 'Year',
    'AverageBpm', 'DateAdded', 'BitRate', 'SampleRate', 'Comments',
    'PlayCount', 'Rating', 'Location', 'Remixer', 'Tonality', 'Label', 'Mix',
]

for tid in sorted(track_info.keys()):
    info = track_info[tid]
    attribs = {'TrackID': str(tid)}
    for attr in TRACK_ATTR_ORDER[1:]:  # skip TrackID, already set
        attribs[attr] = info.get(attr, '')
    ET.SubElement(collection, 'TRACK', **attribs)

# ---- Build playlist tree ----
playlists_el = ET.SubElement(root, 'PLAYLISTS')

def build_tree(parent_el, folder_prefix, all_playlists):
    direct_playlists = []
    subfolders = defaultdict(list)
    for (fp, pl_name, track_locs) in all_playlists:
        if fp == folder_prefix:
            direct_playlists.append((pl_name, track_locs))
        elif len(fp) > len(folder_prefix) and fp[:len(folder_prefix)] == folder_prefix:
            subfolders[fp[len(folder_prefix)]].append((fp, pl_name, track_locs))
    for folder_name in sorted(subfolders.keys()):
        folder_el = ET.SubElement(parent_el, 'NODE', Type='0', Name=folder_name)
        build_tree(folder_el, folder_prefix + (folder_name,), subfolders[folder_name])
        folder_el.set('Count', str(len(list(folder_el))))
    for pl_name, track_locs in sorted(direct_playlists, key=lambda x: x[0]):
        node = ET.SubElement(parent_el, 'NODE', Type='1', Name=pl_name,
                             KeyType='0', Entries=str(len(track_locs)))
        for loc in track_locs:
            ET.SubElement(node, 'TRACK', Key=str(track_map[loc]))

root_node   = ET.SubElement(playlists_el, 'NODE', Type='0', Name='ROOT')
j_drive_node = ET.SubElement(root_node,  'NODE', Type='0', Name='J Drive Playlists')
build_tree(j_drive_node, (), playlists)
j_drive_node.set('Count', str(len(list(j_drive_node))))
root_node.set('Count', '1')

# ---- Pretty-print and write ----
def indent(elem, level=0):
    i = '\n' + '  ' * level
    if len(elem):
        if not elem.text or not elem.text.strip(): elem.text = i + '  '
        if not elem.tail or not elem.tail.strip(): elem.tail = i
        for child in elem: indent(child, level + 1)
        if not child.tail or not child.tail.strip(): child.tail = i
    else:
        if level and (not elem.tail or not elem.tail.strip()): elem.tail = i

indent(root)
with open(OUTPUT_XML, 'w', encoding='utf-8') as f:
    f.write('<?xml version="1.0" encoding="UTF-8"?>\n')
    ET.ElementTree(root).write(f, encoding='unicode', xml_declaration=False)

print(f"\nWritten: {OUTPUT_XML}")
