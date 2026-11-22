#!/usr/bin/env python3
"""Render the shared firmware drawing commands on the host; not a device capture."""
import argparse
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    args = parser.parse_args()
    fonts = [Path('/System/Library/Fonts/Helvetica.ttc'), Path('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')]
    font = next((p for p in fonts if p.is_file()), None)
    if not font:
        parser.error('Install Helvetica or DejaVu Sans for host previews')
    for path in sorted(args.directory.glob('*.json')):
        data = json.loads(path.read_text())
        if data.get('kind') != 'host-render-not-device-screenshot':
            continue
        image = Image.new('RGB', (320*3, 240*3))
        draw = ImageDraw.Draw(image)
        for op in data['ops']:
            if 'rect' in op:
                x,y,w,h = op['rect']
                if w <= 0 or h <= 0:
                    continue
                draw.rectangle((x*3,y*3,(x+w)*3-1,(y+h)*3-1), fill=op['color'])
            else:
                x,y,w,h = op['box']
                f = ImageFont.truetype(str(font), round(op['size']*3))
                # Match a single-line firmware box: clip long captions inside it.
                layer = Image.new('RGBA', (max(1,w*3), max(1,h*3)))
                ImageDraw.Draw(layer).text((0,0), op['text'], font=f, fill=op['color'], anchor='lt')
                image.paste(layer,(x*3,y*3),layer)
        image.save(path.with_suffix('.png'))

if __name__ == '__main__':
    main()
