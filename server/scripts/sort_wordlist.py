import sys

filename = sys.argv[1]
file = open(filename, 'r')
lines = [''.join(i.split()) for i in file]
lines.sort()
lines = [i for i in lines if i[0] != '#']
lines = [i+'\n' for i in lines if len(i) > 1]
file.close()

output = 'output.txt'
file = open(output, 'w')
file.writelines(lines)
file.close()
