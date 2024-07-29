class Example {
  // 方法定义
  method() { // 对应 Rust 中的 MethodDefinition
    console.log("This is a method.");
  }

  // 静态方法定义
  static staticMethod() { // 对应 Rust 中的 MethodDefinition
    console.log("This is a static method.");
  }

  // getter 方法定义
  get value() { // 对应 Rust 中的 MethodDefinition
    return this._value;
  }

  // setter 方法定义
  set value(val) { // 对应 Rust 中的 MethodDefinition
    this._value = val;
  }
}