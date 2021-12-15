Scaffolding originally copied from: https://github.com/pd-andy/advent-of-code-elm

#To run elm

- npm install the modules
- create a session.json file with the value of the session in the cookie found in the inspector
  - make sure to put it in quotes!
- create the root data directory for the year in `./data/[year]`
- generate the day from a template with `node ./generate.js -y 2020 -d 1`
- run the script with `npm run solve -- -y 2020 -d 1 -p 1`
