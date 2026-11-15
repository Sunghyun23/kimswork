"""User-declared pad locations, independent of MIDI-note routing and Q-Links."""
from PIL import Image, ImageDraw


def install(g, tui, params, out):
    O = g.obj

    def get(o, key):
        return next(v for k, v in o if k == key)

    def put(o, key, value):
        for i, (k, _) in enumerate(o):
            if k == key:
                o[i] = (k, value)
                return
        o.append((key, value))

    def cd(w):
        return get(w, 'componentData')

    pd = get(tui, 'pageData')
    entries = get(get(pd, 'componentDefinitions'), 'localComponentDefinitions')
    defs = {get(e, 'key'): get(e, 'value') for e in entries}
    by = {p.name: p for p in params}

    def add(key, comp):
        entries.append(O(('key', key), ('value', comp)))
        defs[key] = comp

    def label(text, bounds, size=24, handle=None, colour='fff2e2c4'):
        data = g.label_data('Value' if handle else 'Text', colour, size,
                            'horizontallyCentred verticallyCentred', None if handle else text)
        return g.entry(text or 'Pad position', 'Label', data, bounds, 'No', 'Show', 'Always',
                       remap=[('Data', handle or 'Parameter 0')])

    def hit(name, handle, bounds, value, count):
        return g.entry(name, 'Button', g.button_data('button-hit.png', 'button-hit.png', value, count),
                       bounds, 'Yes', 'Show', 'Always', remap=[] if handle == 'Data' else [('Data', handle)])

    def instance(key, bounds='0 0 1280 628', invalid=None):
        return g.entry(key, key, O(('version', 1), ('handleName', 'Data')), bounds,
                       'Children', 'Hide' if invalid else 'Show', 'Always',
                       remap=[('Data', 'Parameter 0')], invalidating=invalid)

    def art(name, bounds):
        return g.entry(name, 'Image', O(('version', 2), ('imageType', 'Regular'),
                       ('colour', '0'), ('image', name)), bounds, 'No', 'Show', 'Always')

    def face(w, h, stem, dark=False):
        for state in ('off', 'on'):
            im = Image.new('RGBA', (w, h))
            d = ImageDraw.Draw(im)
            d.rounded_rectangle((1, 1, w-2, h-2), radius=5,
                                fill=('#a86c37' if state == 'on' else '#303035') if dark
                                else ('#ac713c' if state == 'on' else '#dbc594'),
                                outline='#8d704c', width=2)
            im.save(out / f'{stem}-{state}.png')

    face(120, 44, 'position-field')
    face(146, 44, 'position-learn')
    face(44, 44, 'position-bank', True)
    face(88, 72, 'position-cell', True)

    # A read-only film strip draws all 129 saved values at native size. The
    # original transparent hold Slider remains above every decorative layer.
    strip = Image.new('RGBA', (40, 40 * 129))
    for value in range(1, 129):
        im = Image.new('RGBA', (40, 40))
        d = ImageDraw.Draw(im)
        pad = (value - 1) % 16
        for n in range(16):
            x, y = (n % 4) * 10, (3 - n // 4) * 10
            d.rounded_rectangle((x+1, y+1, x+8, y+8), radius=1,
                                fill='#9c5429' if n == pad else '#b3a482')
        strip.alpha_composite(im, (0, value * 40))
    strip.save(out / 'pad-position-strip.png')

    # Share immutable cell definitions across sixteen slots; bind their two
    # handles per instance instead of duplicating thousands of button trees.
    for value in range(1, 129):
        b, pad = divmod(value - 1, 16)
        cell = [g.entry('Pad face', 'Button',
                        g.button_data('position-cell-on.png', 'position-cell-off.png', value, 129),
                        '0 0 88 72', 'No', 'Show', 'Always', remap=[]),
                label(f'{chr(65+b)}{pad+1}', '0 0 88 72', 28),
                hit('Pad face touch', 'Data', '0 0 88 72', value, 129)]
        add(f'DTPadPositionCell{value}', g.component_definition(
            [g.action('Mouse Down', 'Toggle Switch', 'Close', '')], cell, '00000000'))

    main = get(get(pd, 'tabs')[0], 'componentDefinition')
    for w in get(main, 'componentsData'):
        name = get(cd(w), 'name')
        typ = get(cd(w), 'type')
        if not name.startswith('Chord ') or typ not in defs or 'DTPad' not in typ:
            continue
        slot = int(name.split()[1])
        p = by[f'Pad {slot} Position']
        children = get(defs[typ], 'componentsData')
        for c in children:
            if get(cd(c), 'type') == 'Label' and get(get(cd(c), 'data'), 'type') == 'Value':
                put(get(c, 'bounds'), 'bounds', '4 18 158 48')
                put(get(get(get(cd(c), 'data'), 'textStyle'), 'font'), 'height', 36.)
        grid = g.entry(f'Position grid {slot}', 'Knob',
                       g.knob_filmstrip_data('pad-position-strip.png', 129),
                       '172 25 40 40', 'No', 'Show', 'Always', remap=[('Data', p.handle)])
        caption = label('', '158 2 60 22', 18, p.handle, 'ff352d21')
        # Hide even the word Unassigned; an unregistered pad has no location.
        for item in (grid, caption):
            b = get(item, 'bounds')
            put(b, 'version', 2)
            put(b, 'showWhenDataModelInvalid', 'Hide')
            put(b, 'additionalInvalidatingHandles', [f'RangedEnabling/1/128/129/{p.handle}'])
        children[1:1] = [grid, caption]

    widgets = get(defs['Editor_DTAllChords'], 'componentsData')
    # Free the rightmost area without touching Root/Type/Voice/Octave or MIDI.
    for w in widgets:
        if get(cd(w), 'name').startswith('LEARN '):
            x, y, width, height = map(int, get(get(w, 'bounds'), 'bounds').split())
            put(get(w, 'bounds'), 'bounds', f'1106 {y} 146 44')
            if get(cd(w), 'type') == 'Button':
                data = get(cd(w), 'data')
                if get(data, 'offImage') != 'button-hit.png':
                    put(data, 'onImage', 'position-learn-on.png')
                    put(data, 'offImage', 'position-learn-off.png')

    panels = []
    for slot in range(1, 17):
        p = by[f'Pad {slot} Position']
        opened = f'Local/2/padPositionOpen{slot}'
        bank = f'Local/8/padPositionBank{slot}'
        page_condition = [f'IndexedEnabling/{(slot-1)//4}/4/{by["Chord Pad Page"].handle}']
        y = 116 + (slot-1) % 4 * 100
        field = [g.entry('Position face', 'Button',
                         g.button_data('position-field-on.png', 'position-field-off.png', 1, 2),
                         '0 0 120 44', 'No', 'Show', 'Always', remap=[('Data', opened)]),
                 label('', '0 0 120 44', 21, p.handle, 'ff352d21'),
                 hit('Position face touch', opened, '0 0 120 44', 1, 2)]
        key = f'DTPadPositionField{slot}'
        add(key, g.component_definition([], field, '00000000'))
        widgets.append(instance(key, f'972 {y+17} 120 44', page_condition))
        title = label('PAD LOCATION', f'972 {y-5} 120 20', 14, colour='ff352d21')
        put(get(title, 'bounds'), 'version', 2)
        put(get(title, 'bounds'), 'additionalInvalidatingHandles', page_condition)
        put(get(title, 'bounds'), 'showWhenDataModelInvalid', 'Hide')
        widgets.append(title)

        ws = [art('sidebar-panel.png', '0 0 426 628'),
              hit('Dismiss outside', opened, '426 0 854 628', 0, 2),
              art('sidebar-header.png', '0 0 426 58'),
              label(f'SLOT {slot} · PAD LOCATION', '10 0 356 58', 25),
              label('×', '368 0 58 58', 36), hit('Close', opened, '368 0 58 58', 0, 2),
              label('Manual hint · not detected by MPC', '12 65 402 28', 20),
              label('Choose location, then use LEARN.', '12 94 402 28', 20)]
        for b in range(8):
            bounds = f'{25+b*47} 132 44 44'
            ws += [g.entry('Bank face', 'Button', g.button_data('position-bank-on.png', 'position-bank-off.png', b, 8),
                           bounds, 'No', 'Show', 'Always', remap=[('Data', bank)]),
                   label(chr(65+b), bounds, 25), hit('Bank face touch', bank, bounds, b, 8)]
            cells = []
            for pad in range(16):
                value = b*16+pad+1
                ck = f'DTPadPositionCell{value}'
                cells.append(g.entry(ck, ck, O(('version', 1), ('handleName', 'Data')),
                                     f'{25+(pad%4)*95} {194+(3-pad//4)*82} 88 72',
                                     'Children', 'Show', 'Always',
                                     remap=[('Data', p.handle), ('Close', opened)]))
            bk = f'DTPadPositionBank{slot}-{b}'
            add(bk, g.component_definition([], cells, '00000000', ignore_mouse=True))
            ws.append(instance(bk, invalid=[f'IndexedEnabling/{b}/8/{bank}']))
        clear = [label('CLEAR LOCATION', '0 0 376 52', 24),
                 hit('Clear location', p.handle, '0 0 376 52', 0, 129)]
        ck = f'DTPadPositionClear{slot}'
        add(ck, g.component_definition([g.action('Mouse Down', 'Toggle Switch', opened, '')], clear, '00000000'))
        ws.append(instance(ck, '25 554 376 52'))
        panel = g.component_definition([], ws, '90000000')
        put(panel, 'version', 4)
        panel.insert(next(i for i, (k, _) in enumerate(panel) if k == 'componentsData'), ('hideQLinkBounds', True))
        key = f'DTPadPositionPanel{slot}'
        add(key, panel)
        panels.append(instance(key, invalid=[f'IndexedEnabling/1/2/{opened}'] + page_condition))
    # MPC only suppresses native Q-Link bounds for modal siblings at the tab
    # root; nesting this inside the editor body leaves its guide visible.
    chord_tab = next(t for t in get(pd, 'tabs') if get(t, 'tabName') == 'CHORD')
    get(get(chord_tab, 'componentDefinition'), 'componentsData').extend(panels)
