function longest(list) {
  let prefix = list[0];

  for (let i = 0; i < list.length; i++) {
    console.log('str: ', list[i]);
    while (list[i].indexOf(prefix) !== 0) {
      prefix = prefix.slice(0, prefix.length - 1);
      console.log('    prefix: ', prefix);

      if (prefix === '') {
        return '';
      }
    }
  }

  console.log(prefix);
  return prefix;
}

function longest1(list) {
  let str = list[0];

  for (let i = 0; i < str.length; i++) {
    const char = str[i];

    for (let j = 0; j < list.length; j++) {
      const jstr = list[j];
      if (jstr[i] !== char) {
        return str.slice(0, i);
      }
    }
  }

  return str;
}

const t = longest1(['flower', 'flow', 'flowerp']);
console.log(t);
