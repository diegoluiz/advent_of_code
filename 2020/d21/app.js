const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync('d21/input.txt', 'utf8').split('\n');

function part1() {
    const foods = input.map(x => {
        const [b, c] = x.split('(');
        const ingredients = b.split(' ').filter(x => x);
        const allergens = c.replace('contains', '').replace(')', '').replace(/\s/g, '').split(',').filter(x => x);

        return {
            ingredients,
            allergens
        };
    });

    const foodsByAllergen = {};
    foods.forEach(food => {
        food.allergens.forEach(allergen => {
            if (!foodsByAllergen[allergen]) {
                foodsByAllergen[allergen] = [];
            }
            foodsByAllergen[allergen].push(food);
        });
    });

    const foodsWithAllergen = {};
    while (true) {
        const temp = _.chain(foodsByAllergen)
            .map((foods, allergen) => ({ allergen, foods }))
            .filter((x) => x.foods.length >= 2)
            .map((v) => {
                const ingredients = _.intersection(...v.foods.map(x => x.ingredients));
                if (ingredients.length !== 1) {
                    return null;
                }
                return { ingredient: ingredients[0], allergen: v.allergen };
            })
            .filter()
            .value();

        if (temp.length < 1) {
            break;
        }

        temp.forEach(x => {
            foodsWithAllergen[[x.allergen]] = x.ingredient;
            foodsByAllergen[[x.allergen]].forEach(food => {
                food.ingredients = food.ingredients.filter(ingredient => ingredient != x.ingredient);
            });
            delete foodsByAllergen[[x.allergen]];
        });

        _.forEach(foodsWithAllergen, (v, k) => {
            foods.forEach(food => {
                food.ingredients = food.ingredients.filter(ingredient => ingredient != v);
                food.allergens = food.allergens.filter(allergen => allergen != k);
            });
        });
    }

    const temp = _.chain(foods)
        .filter((x) => x.allergens.length === 1 && x.ingredients.length === 1)
        .map((v) => {
            return { ingredient: v.ingredients[0], allergen: v.allergens };
        })
        .filter()
        .value();


    temp.forEach(x => {
        foods.forEach(food => {
            food.ingredients = food.ingredients.filter(ingredient => ingredient != x.ingredient);
        });
        delete foodsByAllergen[[x.allergen]];
    });

    const list = _.flatMap(foods, x => x.ingredients);
    return list.length;
}

function part2() {
    const foods = input.map(x => {
        const [b, c] = x.split('(');
        const ingredients = b.split(' ').filter(x => x);
        const allergens = c.replace('contains', '').replace(')', '').replace(/\s/g, '').split(',').filter(x => x);

        return {
            ingredients,
            allergens
        };
    });

    const foodsByAllergen = {};
    foods.forEach(food => {
        food.allergens.forEach(allergen => {
            if (!foodsByAllergen[allergen]) {
                foodsByAllergen[allergen] = [];
            }
            foodsByAllergen[allergen].push(food);
        });
    });

    const foodsWithAllergen = {};
    while (true) {
        const temp = _.chain(foodsByAllergen)
            .map((foods, allergen) => ({ allergen, foods }))
            .filter((x) => x.foods.length >= 2)
            .map((v) => {
                const ingredients = _.intersection(...v.foods.map(x => x.ingredients));
                if (ingredients.length !== 1) {
                    return null;
                }
                return { ingredient: ingredients[0], allergen: v.allergen };
            })
            .filter()
            .value();

        if (temp.length < 1) {
            break;
        }

        temp.forEach(x => {
            foodsWithAllergen[[x.allergen]] = x.ingredient;
            foodsByAllergen[[x.allergen]].forEach(food => {
                food.ingredients = food.ingredients.filter(ingredient => ingredient != x.ingredient);
            });
            delete foodsByAllergen[[x.allergen]];
        });

        _.forEach(foodsWithAllergen, (v, k) => {
            foods.forEach(food => {
                food.ingredients = food.ingredients.filter(ingredient => ingredient != v);
                food.allergens = food.allergens.filter(allergen => allergen != k);
            });
        });
    }

    const a = _.chain(foodsWithAllergen)
        .map((v, k) => ({ k, v }))
        .sortBy(x => x.k)
        .map(x => x.v)
        .value()
        .join(',');

    return a;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
