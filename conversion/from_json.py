import json
# note that this is highly manual and wont work for just any midi
# do not use if you dont know what to do

with open('minuet_in_g.json') as f:
    data = json.load(f)

delta = 24
length = (int(data['notes'][-1]["time"][:-1]) + int(data['notes'][-1]["duration"][:-1])) // delta

notes = [[] for _ in range(length + 1)]

for note in data['notes']:
    t = int(note['time'][:-1]) // delta
    d = int(note['duration'][:-1]) // delta
    n = note['midiNote']
    try:
        last = notes[t - 1].index(n)
        notes[t - 1][last] *= -1
    except ValueError:
        ...
    for i in range(d + 1):
        notes[t+i].append(n)
    
    
with open('../auto_save.syb', 'wb') as syb:
    syb.write(b'\x02\x0E\x05\x04')
    for _ in range(7):
        syb.write(b'\x00\x01\xFF\x01')
    for _ in range(7):
        syb.write(b'\x00\x04\xFF\x01')
    for t, line in enumerate(notes):
        for _ in range(2):
            if len(line) > 7:
                print(f'skipped {len(line)} @ {t}!');
                line = []
            for i, note in enumerate(line):
                if note < 0:
                    note *= -1
                    short = True
                else:
                    short = False
                n = (note + 12 - 3) % 12
                if n == 0:
                    n = 1
                o = note // 12
                if n >= 10:
                    o -= 1
                if  short:
                    o += 128  # !
                syb.write(n.to_bytes(1, 'big'))
                syb.write(o.to_bytes(1, 'big'))
            for _ in range(7-len(line)):
                syb.write(b'\x00')
                