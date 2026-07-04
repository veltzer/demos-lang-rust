# Temperature Converter Exercise

## Learning Objectives

### Primary Goals
- Understand basic input/output in Rust
- Work with floating-point numbers
- Implement error handling
- Create and use functions
- Practice type conversion

### The Task
Create a program that converts temperatures from Fahrenheit to Celsius. The program should:
1. Prompt the user for input
2. Accept a temperature in Fahrenheit
3. Convert it to Celsius
4. Display the result with one decimal place

### Key Concepts Covered
1 **User Input**
    - Using `std::io`
    - Reading from stdin
    - String manipulation (trim)
2 **Type Conversion**
    - String to number parsing
    - Using the `parse()` method
    - Error handling with `match`

3 **Floating Point Operations**
    - Working with `f64` type
    - Mathematical operations
    - Formatting decimal places

4 **Function Implementation**
    - Creating a pure function
    - Return types
    - Function documentation

### Common Pitfalls to Avoid
- Forgetting to handle invalid input
- Not trimming whitespace from input
- Missing error handling
- Integer division instead of floating-point
- Not formatting the output properly

### Extension Ideas
1 Add support for Celsius to Fahrenheit conversion
2 Add Kelvin conversion
3 Implement input validation for reasonable temperature ranges
4 Add a loop to convert multiple temperatures
5 Create a temperature comparison feature

### Testing Your Implementation
- Test with whole numbers
- Test with decimal numbers
- Test with negative temperatures
- Test with invalid input
- Test edge cases (-40°F = -40°C)

### Real-World Applications
- Weather applications
- Scientific calculations
- International communication
- Cooking and baking
- Industrial processes

### Assessment Criteria
Your solution should demonstrate:
1. Correct mathematical conversion
2. Robust error handling
3. Clear user interface
4. Well-structured code
5. Proper documentation
