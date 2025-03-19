// Define a discriminated union type
type MyResult<T> =
    | { success: true; value: T; message: string }
    | { success: false; error: string };

// Function that returns different results based on input
function processValue<T>(input: T | null | undefined): MyResult<T> {
    // Type narrowing - behavior changes based on type
    if (input === null) {
        return { success: false, error: "Input was null" };
    } else if (input === undefined) {
        return { success: false, error: "Input was undefined" };
    } else {
        return {
            success: true,
            value: input,
            message: `Successfully processed value of type ${typeof input}`
        };
    }
}

// Function that uses the type information to behave differently
function handleResult<T>(result: MyResult<T>): string {
    // The compiler knows which properties are available based on the discriminant
    if (result.success === true) {
        // Only accessible if success is true
        return `SUCCESS: ${result.message}, value: ${JSON.stringify(result.value)}`;
    } else {
        // Only accessible if success is false
        return `ERROR: ${result.error}`;
    }
}

// Test with different inputs
const stringResult = processValue("Hello TypeScript");
const numberResult = processValue(42);
const objectResult = processValue({ name: "TypeScript", version: 5 });
const nullResult = processValue(null);
const undefinedResult = processValue(undefined);

// Print results
console.log(handleResult(stringResult));
console.log(handleResult(numberResult));
console.log(handleResult(objectResult));
console.log(handleResult(nullResult));
console.log(handleResult(undefinedResult));

// This would cause a type error if uncommented:
// console.log(stringResult.error);  // Error: Property 'error' doesn't exist on type '{ success: true; value: string; message: string; }'
