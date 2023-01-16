import mido
# note that this is highly manual and wont work for just any midi
# do not use if you dont know what to do
# the midis here are from chromeexperiments music maker, 
# so they have a guaranteed tone length of 1

mid = mido.MidiFile('rush_e.mid')
notes_lines = [[]]
for msg in mid.play():
    # print(msg.__dict__, msg.time)
    if msg.time > 0:
        print('>> x' + str(msg.time / 0.09375))
        for _ in range(int(msg.time / 0.09375)):
            notes_lines.append([])
    if msg.type == 'note_on':
        print(msg.note, end=' ')
        notes_lines[-1].append(msg.note)
    
with open('converted.syb', 'wb') as syb:
    syb.write(b'\x02\x0E\x0A\x04')
    for _ in range(14):
        syb.write(b'\x00\x01\xFF\x01')
    for t, line in enumerate(notes_lines):
        if len(line) > 14:
            print(f'skipped {len(line)} @ {t}!');
            line = []
        for i, note in enumerate(line):
            n = (note + 12 - 3) % 12
            if n == 0:
                n = 1
            o = note // 12
            if n >= 10:
                o -= 1
            o += 128
            syb.write(n.to_bytes(1, 'big'))
            syb.write(o.to_bytes(1, 'big'))
        for _ in range(14-len(line)):
            syb.write(b'\x00')
                