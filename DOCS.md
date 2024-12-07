# Snarp Language Documentation

## Overview
Snarp is a simple scripting written in rust language that supports defining and calling functions, printing messages, and creating windows. This documentation provides an overview of the syntax and built-in functions available in Snarp.

## Syntax

### Function Definition
Functions in Snarp are defined using the `func` keyword followed by the function name, parentheses, and a block of code enclosed in curly braces.

```sp
func functionName() {
    // function body
}
```

### Function Call
Functions are called by their name followed by parentheses.

```sp
functionName();
```

### Print Statement
The `print` function is used to print messages to the console. The message should be enclosed in double quotes.

```sp
print("Hello, World!");
```

### Create Window
The `makeWindow` function is used to create a window with a specified title and platform. The title and platform should be enclosed in double quotes.

```sp
makeWindow("Window Title", 500, 500 );
```

## Built-in Functions

### `print`
Prints a message to the console.

**Syntax:**
```sp
print("message");
```

**Example:**
```sp
print("Hello, World!");
```

### `makeWindow`
Creates a window with the specified title and height and width. It is in a variable.

**Syntax:**
```sp
makeWindow("My Window", 500, 500);
```

**Example:**
```sp
windowVar = makeWindow("My Window", 500, 500);
```

## Example Code
Here is an example of a Snarp script that defines a function, prints messages, and creates a window.

```sp
func greet() {
    print("this is a");
    print("function!");
}

print("rgf");
greet();

myWindow = makeWindow("My Window", 500, 500);
```

