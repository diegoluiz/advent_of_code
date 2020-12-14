module.exports = {
    "env": {
        "browser": false,
        "es2021": true
    },
    "extends": "eslint:recommended",
    "parserOptions": {
        "ecmaVersion": 12,
        "sourceType": "module"
    },
    "rules": {
        "semi": 1
    },
    "globals": {
      "require": true,
      "process": true
    },
};
