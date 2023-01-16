import mido
# note that this is highly manual and wont work for just any midi
# the midis here are from chromeexperiments music maker, so they have a guaranteed tone length of 1

mid = mido.MidiFile('rush_e.mid')
print(mid.__dict__)
notes_lines = [[]]
for msg in mid.play():
    # print(msg.__dict__, msg.time)
    if msg.type == 'note_on':
        print(msg.note, end=' ')
        notes_lines[-1].append(msg.note)
        if msg.time > 0:
            print(' >>' + str(msg.time))
            notes_lines.append([])
    
with open('converted.syb'; 'wb') as syb:
    syb.write(b'\x02\x0E\x08\x04')
    for _ in range(14):
        syb.write(b'\x00\x01\xFF\x01')
    