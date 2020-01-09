input = open("day4_input")

from datetime import datetime
import re

class entry_obj :
    def __init__(self, date, entry):
        self.date = date
        self.entry = entry
    def __repr__(self) :
        return "%s %s" % (self.date, self.entry)

class guard_obj :
    def __init__(self, id) :
        self.id = id
        self.sleep = 0
        self.startlist = []
        self.stoplist = []
        self.mostCommonmin= -1
        self.freq = -1
list = []
for line in input :
    line = re.split('\[|\]', line)
    date = line[1]
    entry = line[2]
    date_obj = datetime.strptime(date, "%Y-%m-%d %H:%M")
    list.append(entry_obj(date_obj, entry))


sorted_list = sorted(list, key=lambda obj : obj.date)
print sorted_list

guard = None
start = 0
stop = 0
guards = {}
for data in sorted_list :
    entry = data.entry
    date = data.date
    if "Guard" in entry :
        id = re.findall(r"\d+", entry)[0]
        if id in guards :
            guard = guards[id]
        else :
            guard = guard_obj(id)
            guards[id] = guard
    elif "wakes up" in entry:
        guard.stoplist.append(date)
    elif "falls asleep" in entry:
        guard.startlist.append(date)

maxsleep = 0
maxguard = None
maxfreq = -1
maxfreqguard = None
for guardkey in guards :
    sleeptime = {}
    guard = guards[guardkey]
    for i in range(len(guard.startlist)) :
        stop = guard.stoplist[i].minute
        start = guard.startlist[i].minute
        guard.sleep += (stop - start)
        if guard.sleep > maxsleep :
            maxsleep = guard.sleep
            maxguard = guard
        for j in range(stop - start):
            if start+j in sleeptime:
                sleeptime[start + j] += 1
            else :
                sleeptime[start + j] = 1
        min = max(sleeptime, key=sleeptime.get)
        guard.mostCommonmin = min
        guard.freq = sleeptime[min]
        if maxfreq < guard.freq :
            maxfreq = guard.freq
            maxfreqguard = guard.id

print "Winner :"
print maxguard.id, maxsleep
print "Winner 2 is :" , maxfreq, maxfreqguard, guards[maxfreqguard].mostCommonmin
print "right answer :" , int(maxfreqguard) * int(guards[maxfreqguard].mostCommonmin)

print "==========="

print "id with most minutes ", maxguard.id
print "most overlapped minutes ", int(maxguard.mostCommonmin)
print "sleeptime ", maxsleep
print "right answer ", int(maxguard.mostCommonmin) * int(maxguard.id)
