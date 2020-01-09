
from collections import defaultdict
import re

with open('day7_input', 'r') as f:
    reqs = defaultdict(set)
    steps = set()
    for line in f:
        line = line.rstrip('\n')
        matches = re.match(r'Step ([A-Z]) must be finished before step ([A-Z]) can begin.', line)
        req, step = matches.groups()
        reqs[step].add(req)
        steps.add(req)
        steps.add(step)

    sortedSteps = sorted(steps)
    
    import sys
    print 'P1: ',
    done = set()
    while sortedSteps:
        for step in sortedSteps:
            deps = reqs[step]
            if len(deps - done) == 0:
                sys.stdout.write(step) 
                sortedSteps.remove(step)
                done.add(step)
                break

    print 

    sortedSteps = sorted(steps)

    def getTime(step):
        baseTimePerStep = 60
        stepVal = ord(step) - ord('A') + 1
        return baseTimePerStep + stepVal

    maxWorkers = 5
    workers = []
    stepsWorked = []
    time = 0
    done = set()
    res = ''
    while sortedSteps:

        # Find completed steps and tick non-completed steps
        deleteIndexes = []
        for i, (worker, step) in enumerate(zip(workers, stepsWorked)):
            worker -= 1
            if worker == 0:
                # print('Step {} is done at time {}'.format(step, time))

                deleteIndexes.append(i)

                sortedSteps.remove(step)
                done.add(step)
                res += step

            workers[i] = worker

        for i in deleteIndexes:
            del workers[i]
            del stepsWorked[i]

        # Check if new steps are available to start
        for step in sortedSteps:
            deps = reqs[step]
            canDoStep = len(deps - done) == 0
            if canDoStep and step not in stepsWorked:
                if len(workers) < maxWorkers:
                    workers.append(getTime(step))
                    stepsWorked.append(step)

        time += 1

    print('P2:', time - 1)