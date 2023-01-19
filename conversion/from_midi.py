import mido
# note that this is highly manual and wont work for just any midi
# do not use if you dont know what to do
# the midis here are from chromeexperiments music maker, 
# so they have a guaranteed tone length of 1

mid = mido.MidiFile('levan_polka.mid')
active_notes = []
notes_lines = [[]]
length = 128
for msg in mid.play():
    #print(msg, active_notes)
    if msg.time > 0:
        print('>> x' + str(msg.time / 0.124))
        #length += int(msg.time / 0.124)
        for _ in range(int(msg.time / 0.124)):
            notes_lines.append([])
    if msg.type == 'note_on':
        print(msg.note, end=' ')
        active_notes.append((msg.note, msg.channel))
    if msg.type == 'note_off':
        active_notes.remove((msg.note, msg.channel)) 
    notes_lines[-1] = active_notes.copy()
notes_lines.pop()
# print(length)
with open('converted.syb', 'wb') as syb:
    syb.write(b'\x02\x0E\x08\x04')
    for _ in range(12):
        syb.write(b'\x00\x01\xFF\x01')
    syb.write(b'\x00\x02\x60\x01')  # sqr
    syb.write(b'\x00\x03\x60\x01')  # saw
    for t, line in enumerate(notes_lines):
        if len(line) > 12:
            print(f'skipped {len(line)} @ {t}!');
            line = []
        has_saw = False
        has_sqr = False
        w = 0
        for i, (note, channel) in enumerate(line):
            if channel == 1:
                n = (note + 12 - 3) % 12
                if n == 0:
                    n = 1
                o = note // 12
                if n >= 10:
                    o -= 1
                syb.write(n.to_bytes(1, 'big'))
                syb.write(o.to_bytes(1, 'big'))
                w += 1
            else:
                if note == 35:  # saw
                    has_saw = True
                if note == 39:  # sqr
                    has_sqr = True
        for _ in range(12-w):
            syb.write(b'\x00')
        if has_sqr:
            syb.write(b'\x01')  # C-
            syb.write(b'\x84')  # 4!
        else:
            syb.write(b'\x00')
        if has_saw:
            syb.write(b'\x01')  # C-
            syb.write(b'\x84')  # 4!
        else:
            syb.write(b'\x00')   