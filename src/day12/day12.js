const fs = require('fs');
const readline = require('readline');

const fileName = process.argv[2];

function mod(n, m) {
  return (((n % m) + m) % m);
}

const readInterface = readline.createInterface({
  input: fs.createReadStream(fileName),
  console: false,
});

function processCommand(state, command, value) {
  switch (command) {
    case 'N':
      return { ...state, north: state.north + value };
    case 'S':
      return { ...state, north: state.north - value };
    case 'E':
      return { ...state, east: state.east + value };
    case 'W':
      return { ...state, east: state.east - value };
    case 'L': {
      let newState = { ...state };
      for (let i = 0; i < Math.floor(value / 90); i += 1) {
        newState = {
          east: -newState.north,
          north: newState.east,
        };
      }
      return newState;
    }
    case 'R': {
      let newState = { ...state };
      for (let i = 0; i < Math.floor(value / 90); i += 1) {
        newState = {
          east: newState.north,
          north: -newState.east,
        };
      }
      return newState;
    }
    default:
      return state;
  }
}

function processLine(ship, waypoint, line) {
  const command = line[0];
  const value = parseInt(line.slice(1), 10);

  let nextShipState = ship;

  if (command === 'F') {
    for (let i = 0; i < value; i += 1) {
      nextShipState = {
        east: nextShipState.east + waypoint.east,
        north: nextShipState.north + waypoint.north,
      };
    }
  }

  const nextWaypoint = processCommand(waypoint, command, value);

  return [nextShipState, nextWaypoint];
}

// [E/W, N/S, DIR]
let ship = {
  east: 0,
  north: 0,
};

let waypoint = {
  east: 10,
  north: 1,
};

readInterface.on('line', (line) => {
  [ship, waypoint] = processLine(ship, waypoint, line);
  console.log(ship, waypoint);
});

readInterface.on('close', () => {
  console.log(`Solution: ${Math.abs(ship.east) + Math.abs(ship.north)}`);
});
