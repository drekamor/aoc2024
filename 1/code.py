import pandas as pd

# input = pd.read_csv('input.txt',sep=' ', usecols=['l1','l2'], converters={'file': str})

# input.sort_values(by=['l1'], ascending=True).to_csv('list1.txt', sep=' ', columns={'l1'}, index=False)
# input.sort_values(by=['l2'], ascending=True).to_csv('list2.txt', sep=' ', columns={'l2'}, index=False)

l1 = pd.read_csv('list1.txt',sep=' ', usecols=['l1'])
l2 = pd.read_csv('list2.txt',sep=' ', usecols=['l2'])

diff = 0
score = 0
for i in range(0, 1000):
    diff += abs(l1.get('l1')[i] - l2.get('l2')[i])
    
    c = 0
    for j in range(0, 1000):
        if(l2.get('l2')[j] == l1.get('l1')[i]):
            c+=1
    score += l1.get('l1')[i] * c

print("diff: " + str(diff) + "\nscore: " + str(score))
