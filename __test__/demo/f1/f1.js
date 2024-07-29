// 对象字面量中的属性定义
const obj = {
  property1: 42, 
  property2: "Hello", 
  method() { // 对应 Rust 中的 PropertyDefinition
    console.log("This is a method.");
  }
};

// 类中的属性定义
class Example {
  constructor() {
    this.property1 = 42; // 对应 Rust 中的 PropertyDefinition
    this.property2 = "Hello"; // 对应 Rust 中的 PropertyDefinition
  }

  method() { // 对应 Rust 中的 PropertyDefinition
    console.log("This is a method.");
  }
}