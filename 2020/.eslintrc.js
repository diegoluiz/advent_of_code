module.exports = {
    "env": {
        "node": true,
        "es2021": true
    },
    "extends": "eslint:recommended",
    "parserOptions": {
        "ecmaVersion": 12,
        "sourceType": "module"
    },
    "rules": {
        "semi": 1,
        "no-constant-condition": 0
    },
    "globals": {
      "require": true,
      "process": true
    },
};
