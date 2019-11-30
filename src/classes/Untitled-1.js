const pilarISBNs = [
  { isbn: 9877473062, name: "machuPichu" },
  { isbn: 9876128248, name: "grecia" },
  { isbn: 9876129180, name: "amazonas" },
  { isbn: 9877473712, name: "africa" },
  { isbn: 9877471310, name: "egipto" },
  { isbn: 9877474298, name: "china" },
];

pilarISBNs.sort((a, b) => (a.isbn > b.isbn) ? 1: -1);

console.log(pilarISBNs);
