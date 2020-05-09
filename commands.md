# Commands

### Arithmetic Commands

* ADD [location] [op1] [op2]
* SUB [location] [op1] [op2]
* MUL [location] [op1] [op2]
* DIV [location] [op1] [op2]
* MOD [location] [op1] [op2]

### Rounding Commands

* ROUND [location] [float]
* FLOOR [location] [float]
* CEIL [location] [float]

### Boolean Commands

* AND [location] [op1] [op2]
* OR [location] [op1] [op2]
* XOR [location] [op1] [op2]
* NOT [location] [op1] [op2]

### Variables

* DECL [name] [type]
* SET [location] [literal]
* FREE [name]

### Control Flow

* LABEL [name]
* JMP [label]
* JEQ [label] [op1] [op2]
* JNE [label] [op1] [op2]
* JGT [label] [op1] [op2]
* JLT [label] [op1] [op2]

### String Commands

* PRINT [string]
* INPUT [location]
* CONVERT [location] [var]

### Array Commands

* SLICE [location] [array] [start] [end]
* INDEX [location] [array] [index]
* LEN [location] [array]