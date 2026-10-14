// V8rs Profile-Guided Optimization (PGO) 训练负载
// 全面覆盖解释器核心操作码分发、对象模型、原型链、数组、字符串、正则与 JSON 序列化

// 1. 数值计算与控制流分支 (循环、条件跳转、算术操作码)
function testArithmetic() {
  var acc = 0;
  for (var i = 0; i < 200000; i++) {
    acc = (acc + i * 3 - (i % 7)) ^ (i & 0xFF);
  }
  return acc;
}

// 2. 递归与函数调用栈 (Call / Return / Activation Frame)
function fib(n) {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

// 3. 对象分配、属性存取与原型链 (NamedProperty / Prototype Lookup)
function Point(x, y) {
  this.x = x;
  this.y = y;
}
Point.prototype.distance = function() {
  return Math.sqrt(this.x * this.x + this.y * this.y);
};

function testObjects() {
  var points = [];
  for (var i = 0; i < 10000; i++) {
    var p = new Point(i, i * 2);
    points.push(p);
  }
  var sum = 0;
  for (var j = 0; j < points.length; j++) {
    sum += points[j].distance();
  }
  return sum;
}

// 4. 数组操作 (Array builtins: push, pop)
function testArrays() {
  var arr = [];
  for (var i = 0; i < 20000; i++) {
    arr.push(i);
  }
  var popped = 0;
  for (var j = 0; j < 5000; j++) {
    popped += arr.pop();
  }
  return arr.length + popped;
}

// 5. 字符串与正则表达式 (String / RegExp builtins & execution)
function testStringsAndRegExp() {
  var re = /[a-zA-Z0-9_]+@[a-zA-Z0-9_]+\.[a-zA-Z]{2,4}/g;
  var text = "Contact us at support@example.com or sales@service.org for info.";
  var matches = text.match(re);
  var joined = "";
  for (var i = 0; i < 1000; i++) {
    joined += "item_" + i + ";";
  }
  return (matches ? matches.length : 0) + joined.length;
}

// 6. JSON 序列化 (v8-json)
function testJson() {
  var obj = {
    id: 12345,
    name: "v8rs",
    features: ["pgo", "interpreter", "capi", "fast-alloc"],
    nested: { active: true, ratio: 3.14159 }
  };
  var s = "";
  for (var i = 0; i < 5000; i++) {
    s = JSON.stringify(obj);
  }
  return s.length;
}

// 7. 异常处理 (Try / Catch / Throw)
function testException() {
  var count = 0;
  for (var i = 0; i < 5000; i++) {
    try {
      if (i % 5 === 0) throw new Error("test");
    } catch (e) {
      count++;
    }
  }
  return count;
}

// 执行采样训练
testArithmetic();
fib(22);
testObjects();
testArrays();
testStringsAndRegExp();
testJson();
testException();
