# Refrences

1) Refrences are immutable
2) Mutable refrence can be modify the its value
   but it have one condition that it can have 
   only one refrence to a particular varible

* i.e
```bash
let mut s = String::from("Hello");
let r1 = &mut s;
let r2 = &mut s; // this line will throw error
// in rust two pointers can be assigned to one value
```

3) In case of immutable refrence you can point more than one refrence varible 

* i.e 
```bash
let s = String::from("Hello");
let r1 = &s;
let r2 = &s;
```
