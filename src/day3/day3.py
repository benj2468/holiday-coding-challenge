def readForSlope(x_slope, y_slope, line, line_num):
    if line_num > 0 and line_num % y_slope == 0:
        x = int(line_num * (x_slope / y_slope))
        line_diff = 0

        val = line[x % line_size]
        if val == '#': return 1
    return 0

with open('./input.txt') as file:
    trees1 = trees2 = trees3 = trees4 = trees5 = 0
    line_num = 0
    trees = 0
    line = file.readline()
    line_size = len(line)-1
    while line:
        trees1 += readForSlope(1,1, line, line_num)
        trees2 += readForSlope(3,1, line, line_num)
        trees3 += readForSlope(5,1, line, line_num)
        trees4 += readForSlope(7,1, line, line_num)
        trees5 += readForSlope(1,2, line, line_num)

        line = file.readline()
        line_num += 1


print(trees1 * trees2 * trees3 * trees4 * trees5)
