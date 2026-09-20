const path = Native.args()[0];
const contents = Native.args()[1];

Native.writeTextFile(path, contents);
console.log(Native.readTextFile(path));
