/**
 * This module demonstrates TypeScript's context-sensitive grammar features.
 * It will only print the correct output if the parser correctly handles all
 * the different context-sensitive parts of the language.
 */

// 1. Type parameter ambiguity
// The < token is parsed differently based on context
function demonstrateTypeParameterAmbiguity() {
    // This is parsed as comparison
    const isLessThan = 5 < 10;

    // This is parsed as type parameters
    const identity = <T>(x: T): T => x;

    // This is a tricky case - nested comparisons and type parameters
    const nestedGeneric = identity<Array<number>>(
        [1, 2, 3].filter(x => x < 3)
    );

    return { isLessThan, example: identity<string>("type parameters work"), nestedGeneric };
}

// 2. JSX vs type assertions disambiguation (if JSX enabled)
// Note: This requires JSX to be enabled in tsconfig.json
namespace JsxVsTypeAssertions {
    // Type assertion (angle bracket syntax)
    const typeAssertion = <number>42;

    // This would be JSX in a .tsx file
    // const jsxElement = <div>Hello</div>;

    // This is always a type assertion because it starts with (
    const alwaysTypeAssertion = <number>(42);
}

// 3. Contextual keywords
// Words like 'get', 'set', 'as', etc. have different meanings in different contexts
// 'as' in import statements
import { readFileSync as readFile } from 'fs';

namespace ContextualKeywords {
    // 'get' as property accessor
    const obj = {
        value: 42,
        get x() { return this.value; }
    };

    // 'set' as property accessor
    const obj2 = {
        value: 0,
        set y(v: number) { this.value = v; }
    };

    // 'as' for type assertions
    const num = 42 as number;

    // 'of' in for-of loops vs normal identifier
    function forOfVsIdentifier() {
        const of = "identifier"; // 'of' as identifier
        const items = [1, 2, 3];
        for (const item of items) { // 'of' as keyword
            console.log(item);
        }
        return of;
    }
}

// 4. await/yield contextual handling
async function* demonstrateAwaitAndYield() {
    // 'await' is a keyword in async functions
    const awaitedValue = await Promise.resolve(42);

    // 'yield' is a keyword in generator functions
    yield 1;

    // Both together in async generator
    yield await Promise.resolve(2);

    // 'await' and 'yield' can be identifiers in non-async/generator contexts
    const obj = {
        await: 42,
        yield: "hello"
    };

    return obj;
}

// 5. Arrow function parsing ambiguities
function demonstrateArrowFunctionAmbiguities() {
    // Parentheses are required for empty parameter list
    const noParams = () => "no params";

    // Single parameter without type annotation doesn't need parentheses
    const oneParam = x => x * 2;

    // Single parameter with type annotation needs parentheses
    const oneParamWithType = (x: number) => x * 2;

    // This is a comparison, not an arrow function
    const comparison = 1 < 2;

    // This is a generic arrow function, not a comparison
    const genericArrow = <T>(x: T) => x;

    // Conditional expressions parsing correctly with arrow functions
    const conditional = true ? x => x : y => y * 2;

    return { noParams, oneParam, oneParamWithType, comparison, genericValue: genericArrow(42) };
}

// 6. Conditional types - context sensitivity in the type system
type IsArray<T> = T extends Array<infer U> ? U : never;
type ExtractedType = IsArray<number[]>; // should be 'number'

// 7. Distinguishing between destructuring and object literal
function demonstrateDestructuring() {
    // Object literal
    const obj = { a: 1, b: 2 };

    // Destructuring in variable declaration
    const { a, b } = obj;

    // Destructuring in function parameters
    function process({ a, b }: { a: number, b: number }) {
        return a + b;
    }

    // Nested destructuring
    const nested = { c: { d: 42 } };
    const { c: { d } } = nested;

    return { a, b, processed: process(obj), d };
}

// 8. Grammar ambiguity with tagged template literals
function tag(strings: TemplateStringsArray, ...values: any[]) {
    return strings.map((s, i) => `${s}${values[i] || ''}`).join('');
}

const taggedTemplate = tag`hello ${42} world`;

// Main function to run all examples and verify they work correctly
function main() {
    const results = {
        typeParams: demonstrateTypeParameterAmbiguity(),
        arrowFunctions: demonstrateArrowFunctionAmbiguities(),
        destructuring: demonstrateDestructuring(),
        taggedTemplate
    };

    // This will only print the correct result if the parser correctly handled
    // all the context-sensitive grammar features
    console.log("Context-sensitive grammar parsing successful!");
    console.log(JSON.stringify(results, null, 2));
}

main();
