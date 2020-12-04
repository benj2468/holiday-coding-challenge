const fs = require('fs')
const readline = require('readline')

const regex = /(\w+):([\w#]+)/mg

const validation = {
    'byr': (e) => (/19[2-9][0-9]|200[0-2]/.test(e)),
    'iyr': (e) => (/20[1][0-9]|2020/.test(e)),
    'eyr': (e) => (/202[0-9]|2030/.test(e)),
    'hgt': (e) => (/^(1[5-8][0-9]|19[0-3])cm$|^(59|6[0-9]|7[0-6])in$/.test(e)),
    'hcl': (e) => (/\#[0-9a-f]{6}/.test(e)),
    'ecl': (e) => (/(amb|blu|brn|gry|grn|hzl|oth)/.test(e)),
    'pid': (e) => (/^[0-9]{9}$/.test(e)),
    'cid': (e) => false
}

const checkLine = (passportLine) => {
    return [...passportLine.matchAll(/(\w+):([\w#]+)/g)].reduce((acc, cur) => {
        return acc + (validation[cur[1]](cur[2]) ? 1 : 0)
    }, 0) >= 7 ? 1 : 0
}

const solution = fs
.readFileSync('./input.txt')
.toString()
.split('\n\n')
.filter(line => checkLine(line)).length

console.log(solution)