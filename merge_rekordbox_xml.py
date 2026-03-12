#!/usr/bin/env python3
"""
Merge rekordbox_from_m3u.xml into the main rekordbox.xml export.

Strategy:
- Tracks in the import that already exist in the main library (matched by Location)
  are mapped to their existing TrackID (no duplicate added).
- New tracks are added to COLLECTION with IDs that don't collide with existing ones.
- The import's "J Drive Playlists" folder is added under the main PLAYLISTS ROOT.
"""

import xml.etree.ElementTree as ET
import re
import sys
from pathlib import Path

MAIN_XML    = r'C:\Users\trist\Desktop\playlists-j\rekordbox.xml'
IMPORT_XML  = r'C:\Users\trist\Desktop\playlists-j\rekordbox_from_m3u.xml'
OUTPUT_XML  = r'C:\Users\trist\Desktop\playlists-j\rekordbox_merged.xml'

def normalize_location(loc):
    """Lowercase + strip trailing slashes for comparison."""
    return loc.strip().lower().rstrip('/')

# ---- Parse main XML (streaming to handle 276K lines) ----
print("Parsing main rekordbox.xml...")

# We'll do a regex-based scan for TRACK elements to avoid loading the whole tree
# then do a proper tree parse for the PLAYLISTS section only.

# Step 1: Extract all existing locations and IDs from main XML via regex
# Format: <TRACK TrackID="NNN" ... Location="..." ...
main_id_to_loc = {}   # id -> normalized location
main_loc_to_id = {}   # normalized location -> id
max_main_id = 0

track_id_re  = re.compile(r'TrackID="(\d+)"')
location_re  = re.compile(r'Location="([^"]+)"')

with open(MAIN_XML, 'r', encoding='utf-8') as f:
    in_track = False
    buf = ''
    for line in f:
        stripped = line.strip()
        if stripped.startswith('<TRACK '):
            in_track = True
            buf = stripped
        elif in_track:
            buf += ' ' + stripped

        if in_track and (buf.endswith('/>') or buf.endswith('</TRACK>')):
            # Complete TRACK element — extract ID and Location
            m_id  = track_id_re.search(buf)
            m_loc = location_re.search(buf)
            if m_id and m_loc:
                tid = int(m_id.group(1))
                loc = normalize_location(m_loc.group(1))
                main_id_to_loc[tid] = loc
                main_loc_to_id[loc] = tid
                if tid > max_main_id:
                    max_main_id = tid
            in_track = False
            buf = ''

print(f"  Main library: {len(main_id_to_loc)} tracks, max ID = {max_main_id}")

# ---- Parse import XML (small file, full parse) ----
print("Parsing rekordbox_from_m3u.xml...")
imp_tree = ET.parse(IMPORT_XML)
imp_root = imp_tree.getroot()

imp_collection = imp_root.find('COLLECTION')
imp_playlists  = imp_root.find('PLAYLISTS')

# Build import track lookup
imp_tracks = {}  # import_id -> Element
for t in imp_collection.findall('TRACK'):
    imp_tracks[int(t.get('TrackID'))] = t

print(f"  Import: {len(imp_tracks)} tracks")

# ---- Build ID remapping: import_id -> final_id ----
id_remap = {}        # import_id -> final_id in merged output
new_tracks = []      # (final_id, Name, Location) for tracks not in main

next_new_id = max_main_id + 1

for imp_id, track_el in sorted(imp_tracks.items()):
    loc_raw = track_el.get('Location', '')
    loc_norm = normalize_location(loc_raw)
    if loc_norm in main_loc_to_id:
        # Track already in main library
        id_remap[imp_id] = main_loc_to_id[loc_norm]
    else:
        # New track — assign a new ID
        id_remap[imp_id] = next_new_id
        new_tracks.append((next_new_id, track_el.get('Name', ''), loc_raw))
        # Also register so we don't add duplicates within the import itself
        main_loc_to_id[loc_norm] = next_new_id
        next_new_id += 1

existing_count = len(imp_tracks) - len(new_tracks)
print(f"  {existing_count} tracks already in main library (will reuse their IDs)")
print(f"  {len(new_tracks)} new tracks to add")

# ---- Remap playlist TRACK Key values in import's playlist tree ----
def remap_keys(node):
    for child in list(node):
        if child.tag == 'TRACK':
            old_key = int(child.get('Key'))
            child.set('Key', str(id_remap.get(old_key, old_key)))
        elif child.tag == 'NODE':
            remap_keys(child)

remap_keys(imp_playlists)

# ---- Read main XML into memory, insert new tracks and playlists ----
print("Writing merged XML...")

# Build new track XML strings to insert before </COLLECTION>
new_track_lines = []
for (tid, name, location) in sorted(new_tracks):
    name_escaped = name.replace('&', '&amp;').replace('"', '&quot;').replace('<', '&lt;').replace('>', '&gt;')
    new_track_lines.append(f'    <TRACK TrackID="{tid}" Name="{name_escaped}" Location="{location}" />\n')

# Build the "J Drive Playlists" folder XML
# It will be inserted as a child of ROOT (before ROOT's closing </NODE>)
def escape_attr(v):
    return v.replace('&', '&amp;').replace('"', '&quot;').replace('<', '&lt;').replace('>', '&gt;')

def node_to_xml(node, indent_level=0):
    """Convert an ElementTree node to XML string lines."""
    lines = []
    indent = '      ' + '  ' * indent_level  # 6 spaces = child-of-ROOT level
    tag = node.tag
    attrs = ' '.join(f'{k}="{escape_attr(v)}"' for k, v in node.attrib.items())
    children = list(node)
    if children:
        lines.append(f'{indent}<{tag} {attrs}>\n')
        for child in children:
            lines.extend(node_to_xml(child, indent_level + 1))
        lines.append(f'{indent}</{tag}>\n')
    else:
        lines.append(f'{indent}<{tag} {attrs} />\n')
    return lines

imp_root_node = imp_playlists.find('NODE[@Name="ROOT"]')
j_drive_node  = imp_root_node.find('NODE[@Name="J Drive Playlists"]') if imp_root_node else None

if j_drive_node is None:
    print("WARNING: Could not find 'J Drive Playlists' node in import XML")
    playlist_lines = []
else:
    playlist_lines = node_to_xml(j_drive_node, indent_level=0)

new_collection_count = len(main_id_to_loc) + len(new_tracks)
collection_entries_re = re.compile(r'(<COLLECTION\s+Entries=")[^"]*(")')

# Read all lines into memory (~14MB — fine)
print("  Reading main XML into memory...")
with open(MAIN_XML, 'r', encoding='utf-8') as f:
    lines = f.readlines()

print(f"  {len(lines)} lines read")

# Find insertion points
collection_close_idx = None
playlists_close_idx  = None
root_node_close_idx  = None  # The </NODE> that closes ROOT (last </NODE> before </PLAYLISTS>)

for i, line in enumerate(lines):
    s = line.strip()
    if s == '</COLLECTION>':
        collection_close_idx = i
    elif s == '</PLAYLISTS>':
        playlists_close_idx = i
        # ROOT's closing </NODE> is the last </NODE> before </PLAYLISTS>
        for j in range(i - 1, -1, -1):
            if lines[j].strip() == '</NODE>':
                root_node_close_idx = j
                break
        break

print(f"  </COLLECTION> at line {collection_close_idx}")
print(f"  ROOT </NODE>  at line {root_node_close_idx}")
print(f"  </PLAYLISTS>  at line {playlists_close_idx}")

# Also update ROOT Count attribute
root_count_re = re.compile(r'(<NODE\s+[^>]*Name="ROOT"[^>]*Count=")[^"]*(")')

# Find ROOT node opening line (search further back — it's near <PLAYLISTS> opening)
for i in range(playlists_close_idx - 1, max(0, playlists_close_idx - 20000), -1):
    if 'Name="ROOT"' in lines[i]:
        m = root_count_re.search(lines[i])
        if m:
            # Increment ROOT Count by 1 (adding J Drive Playlists)
            old_count = int(re.search(r'Count="(\d+)"', lines[i]).group(1))
            lines[i] = root_count_re.sub(
                lambda mx: mx.group(1) + str(old_count + 1) + mx.group(2),
                lines[i]
            )
            print(f"  Updated ROOT Count: {old_count} -> {old_count + 1}")
        break

# Fix COLLECTION Entries count
for i, line in enumerate(lines):
    if '<COLLECTION ' in line and 'Entries=' in line:
        lines[i] = collection_entries_re.sub(
            lambda m: m.group(1) + str(new_collection_count) + m.group(2),
            line
        )
        break

# Build final output by splicing in new content
output_lines = (
    lines[:collection_close_idx] +
    new_track_lines +
    lines[collection_close_idx:root_node_close_idx] +
    playlist_lines +
    lines[root_node_close_idx:]
)

with open(OUTPUT_XML, 'w', encoding='utf-8') as fout:
    fout.writelines(output_lines)

print(f"  Wrote {len(output_lines)} lines")
print(f"\nMerged XML: {OUTPUT_XML}")
print(f"Total tracks in merged library: {new_collection_count}")
print(f"New playlists added: 'J Drive Playlists' folder with 152 playlists")
