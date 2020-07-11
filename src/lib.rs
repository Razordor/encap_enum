// Copyright (c) 2020 Jonathan

#![no_std]
#![allow(dead_code)] // tells rust that I'm not testing everything. there is a lot of duplication, so it would be redundant.

/*!
A type-safe encapsulated `enum` useful for interacting with C bitmasks, abbreviating a group of constants with the same type, and an enumerator instance that can be safely assigned too regardless of whether it is in the enum.

Internally `encap_enum` uses a tuple struct, therefore to access the value of a variant use tuple accessor syntax `.0`.

Terms such as `enum`, and `variant` are used throughout the doc even though the internal representation can be different,
and the reason is because rust's canonical `enum` is used internally when omitting explicit declarations. In other words it acts like
an `enum`, yet it is composed of many different constructs.

## Example
The `encap_enum!` allows for bit flags and other common manipulations of data. When the `enum`'s type is omitted such as the example below, the type defaults to `isize`.
```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{
    enum Flags {
        A = 1 + 4 + 8,
        B = 2,
        C = A | B, // C = 15
    }
}
```


The flags can be assigned to constants outside the enum since all the enum members are internally constants:
```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
const VALUE: isize = MoreFlags::Omega.0;
const OTHER: MoreFlags = MoreFlags::Sigma;
encap_enum!{
    enum MoreFlags {
        Omega = 1,
        Sigma = 2,
        Delta = 3,
    }
}
```

Constants outside the enum can be used to initialize variants:
```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){ assert_eq!(TakenFlags::Delta.0, 128);}
const VALUE: u32 = 56;
const OTHER: u32 = 72;
encap_enum!{
    enum TakenFlags: u32 {
        Omega = (enum TakenFlags) VALUE, // Omega = 56
        Sigma = (enum TakenFlags) OTHER, // Sigma = 72
        Delta = (enum TakenFlags) VALUE + (enum TakenFlags) OTHER, // Delta = 128
    }
}
```

The `enum_encap!` macro also supports enumeration like a normal fieldless enum by tapping into rust's own `enum`.
A name must be provided to ensure namespace collision does not occur due to the limitations of interacting with rust's `enum`
```rust
#[macro_use]
extern crate encap_enum;

encap_enum!{
    pub mod flag {
        pub enum OtherFlags: pub isize {
            Alpha,
            Beta,
            Charlie,
        }
    }
}
fn main() {
    println!("{}", flag::OtherFlags::Alpha.0);
}
```
The internal rust enum, which the struct constants draws values from, is under the namespace you set.

## Visibility
The visibility for both the `enum` and variants can be changed:

```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{
    pub mod flag {
        pub enum PublicFlags: pub(in crate) u32 {
            Alpha,
            Beta,
            Charlie,
        }
    }
}
```

## Attributes
Attributes can be used pretty much anywhere in the macro except right under the declaration (See Corner Cases section at the bottom for more details).

## Traits
### Derived Traits
The following traits are derived:
- Debug
- Copy
- Clone
- PartialEq
- Eq
- PartialOrd
- Hash

### Operators
The following operators are implemented:
- BitOr
- Add
- BitAnd
- BitXor
- Div
- Mul
- Shl
- Shr
- Sub
- Rem
- Not
- Neg
- From
- AddAssign
- SubAssign
- MulAssign
- BitAndAssign
- BitOrAssign
- BitXorAssign
- DivAssign
- RemAssign

## Methods
- `iter() -> Iter`: An iterator over all the variants.
- `get_bit(bit:u8)->bool`: query the state of the specified bit. `get_bit`'s visibility depends on the tuple data's visibility.


## Corner Cases
Attributes cannot be placed before the first variant and there are no plans to fix this.
```rust,ignore
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{    
    enum Flag: u32 {
        /// This doc comment causes errors.
        Gamma = 54,
    }
}
```


The `#[repr(C)]` attribute will work and make the enum more ffi compatible, however `#[repr(u8)]`, `#[repr(u16)]`, etc. will not compile because the internal representation that it will apply to is a tuple stuct.
The equivalent of `#[repr(u32)]`, which would apply on an enum would look like this on an `encap_enum!` declaration:

```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{
    #[repr(C)]
    enum Flag: u32 {
        // variants here.
#       Hidden = 0,
    }
}
```
*/

// Provides an implementation to any struct tuple with a single integer field.
#[macro_export]
#[doc(hidden)]
macro_rules! __encap_enum_impl {

    ($name:ident, $type:ty) => {
        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, right: Self) -> Self { Self(self.0 | right.0) }
        }
        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, right: Self) -> Self { Self(self.0 + right.0) }
        }
        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, right: Self) -> Self { Self(self.0 & right.0) }
        }
        impl core::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, right: Self) -> Self { Self(self.0 ^ right.0) }
        }
        impl core::ops::Div for $name {
            type Output = Self;
            fn div(self, right: Self) -> Self { Self(self.0 / right.0) }
        }
        impl core::ops::Mul for $name {
            type Output = Self;
            fn mul(self, right: Self) -> Self { Self(self.0 * right.0) }
        }
        impl core::ops::Shl for $name {
            type Output = Self;
            fn shl(self, right: Self) -> Self { Self(self.0 << right.0) }
        }
        impl core::ops::Shr for $name {
            type Output = Self;
            fn shr(self, right: Self) -> Self { Self(self.0 >> right.0) }
        }
        impl core::ops::Sub for $name {
            type Output = Self;
            fn sub(self, right: Self) -> Self { Self(self.0 - right.0) }
        }
        impl core::ops::Rem for $name {
            type Output = Self;
            fn rem(self, right: Self) -> Self { Self(self.0 % right.0) }
        }
        impl core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self { Self(!self.0) }
        }
        impl core::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self { Self(0 - self.0) }
        }
        impl core::convert::From<$type> for $name {
            fn from(right: $type) -> Self { Self(right) }
        }
        impl core::ops::AddAssign for $name {            
            fn add_assign(&mut self, right: Self) { self.0 += right.0 }
        }
        impl core::ops::SubAssign for $name {            
            fn sub_assign(&mut self, right: Self) { self.0 -= right.0 }
        }
        impl core::ops::MulAssign for $name {            
            fn mul_assign(&mut self, right: Self) { self.0 *= right.0 }
        }
        impl core::ops::BitAndAssign for $name {            
            fn bitand_assign(&mut self, right: Self) { self.0 &= right.0 }
        }
        impl core::ops::BitOrAssign for $name {            
            fn bitor_assign(&mut self, right: Self) { self.0 |= right.0 }
        }
        impl core::ops::BitXorAssign for $name {            
            fn bitxor_assign(&mut self, right: Self) { self.0 ^= right.0 }
        }
        impl core::ops::DivAssign for $name {            
            fn div_assign(&mut self, right: Self) { self.0 /= right.0 }
        }
        impl core::ops::RemAssign for $name {            
            fn rem_assign(&mut self, right: Self) { self.0 %= right.0 }
        }
    }
}



/**
A macro for bit flags and enumerations.

See [crate level docs](https://docs.rs/encap_enum) for full documentation.

## Example
```rust
#[macro_use]
extern crate encap_enum;

encap_enum!{
    enum Flags{
        A = 0x00,
        B = 0x01,
        C = 0x02,
    }
}

fn main() {
    println!("A = {}", Flags::A.0);
}
```
*/
#[macro_export]
macro_rules! encap_enum {
    (
        $(
            $(#[$outer_comment:meta])*
            $outer_vis:vis enum $name:ident : $inner_vis:vis $type:ty {
                $(
                    $val_name:ident =
                        $($li:literal $(- $sub_li:literal)* $(- $li_sub_id:ident)* $(- (enum $type0:path) $li_sub_sid:ident )*)?
                        $($id:ident $(- $sub_id:ident)* $(- $id_sub_li:literal)* $(- (enum $type1:path) $id_sub_sid:ident )*)?                      
                        $((enum $type2:path) $($sid0:ident)? $(- $sid1:ident)* $(- $($sid_sub_li:literal)* )* )*                
                        $((-$id1:ident))?

                        $(|  $bitor_li:literal)*
                        $(+  $add_li:literal)*
                        $(&  $bitand_li:literal)*
                        $(^  $bitxor_li:literal)*
                        $(/  $div_li:literal)*
                        $(*  $mul_li:literal)*
                        $(<< $shl_li:literal)*
                        $(>> $shr_li:literal)*

                        $(|  $bitor_id:ident)*
                        $(+  $add_id:ident)*
                        $(&  $bitand_id:ident)*
                        $(^  $bitxor_id:ident )*
                        $(/  $div_id:ident    )*
                        $(*  $mul_id:ident    )*
                        $(<< $shl_id:ident    )*
                        $(>> $shr_id:ident    )*

                        $(|  (enum $bitor_type_sid:path)  $bitor_sid:ident  )*
                        $(+  (enum $add_type_sid:path)    $add_sid:ident    )*
                        $(&  (enum $bitand_type_sid:path) $bitand_sid:ident )*
                        $(^  (enum $bitxor_type_sid:path) $bitxor_sid:ident )*
                        $(/  (enum $div_type_sid:path)    $div_sid:ident    )*
                        $(*  (enum $mul_type_sid:path)    $mul_sid:ident    )*
                        $(<< (enum $shl_type_sid:path)    $shl_sid:ident    )*
                        $(>> (enum $shr_type_sid:path)    $shr_sid:ident    )*

                    ,$(#[$comment:meta])*
                )+
            }
        )+
    ) => {
        $(
            $(#[$outer_comment])*
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
            $outer_vis struct $name($inner_vis $type);
            __encap_enum_impl!{$name, $type}
            impl $name {
                pub fn iter() -> core::slice::Iter<'static, $type> {

                    const _ARRAY: &[$type] = &[$($name :: $val_name .0,)+];
                    _ARRAY.into_iter()
                }

                $inner_vis fn get_bit(&self, bit:u8) -> bool{

                    self.0 & (1 << bit) != 0
                }

                $(
                    $(#[$comment])*
                    #[allow(non_upper_case_globals)]
                    pub const $val_name: $name = $name (                    
                        $($li $(- $sub_li)* $(- $name :: $li_sub_id .0)* $(- $type0 ($li_sub_sid) .0 )*)?
                        $($name :: $id .0 $(- $name :: $sub_id .0)* $(- $id_sub_li)* $(- $type1 ($id_sub_sid) .0 )*)?
                        $($type2 ($($sid0)? $(- $sid1)*) .0 $(- $($sid_sub_li)* )*)*                    
                        $(- $name :: $id1 .0)?

                        $(|  $bitor_li  )*
                        $(+  $add_li    )*
                        $(&  $bitand_li )*
                        $(^  $bitxor_li )*
                        $(/  $div_li    )*
                        $(*  $mul_li    )*
                        $(<< $shl_li    )*
                        $(>> $shr_li    )*

                        $(|  $name :: $bitor_id .0  )*
                        $(+  $name :: $add_id .0    )*
                        $(&  $name :: $bitand_id .0 )*
                        $(^  $name :: $bitxor_id .0 )*
                        $(/  $name :: $div_id .0    )*
                        $(*  $name :: $mul_id .0    )*
                        $(<< $name :: $shl_id .0    )*
                        $(>> $name :: $shr_id .0    )*

                        $(|  $bitor_type_sid ($bitor_sid)   .0 )*
                        $(+  $add_type_sid ($add_sid)       .0 )*
                        $(&  $bitand_type_sid ($bitand_sid) .0 )*
                        $(^  $bitxor_type_sid($bitxor_sid)  .0 )*
                        $(/  $div_type_sid ($div_sid)       .0 )*
                        $(*  $mul_type_sid ($mul_sid)       .0 )*
                        $(<< $shl_type_sid ($shl_sid)       .0 )*
                        $(>> $shr_type_sid ($shr_sid)       .0 )*
                    );
                )+
            }
        )+
    };
    (
        $(
        $(#[$outer_comment:meta])*
        $outer_vis:vis enum $name:ident {
            $(
                $val_name:ident =
                $($li:literal $(- $sub_li:literal)* $(- $li_sub_id:ident)* $(- (enum $type0:path) $li_sub_sid:ident )*)?
                $($id:ident $(- $sub_id:ident)* $(- $id_sub_li:literal)* $(- (enum $type1:path) $id_sub_sid:ident )*)?                      
                $((enum $type2:path) $($sid0:ident)? $(- $sid1:ident)* $(- $($sid_sub_li:literal)* )* )*                
                $((-$id1:ident))?
                
                $(|  $bitor_li:literal)*
                $(+  $add_li:literal)*
                $(&  $bitand_li:literal)*
                $(^  $bitxor_li:literal)*
                $(/  $div_li:literal)*
                $(*  $mul_li:literal)*
                $(<< $shl_li:literal)*
                $(>> $shr_li:literal)*
                
                $(|  $bitor_id:ident)*
                $(+  $add_id:ident)*
                $(&  $bitand_id:ident)*
                $(^  $bitxor_id:ident )*
                $(/  $div_id:ident    )*
                $(*  $mul_id:ident    )*
                $(<< $shl_id:ident    )*
                $(>> $shr_id:ident    )*

                $(|  (enum $bitor_type_sid:path)  $bitor_sid:ident  )*
                $(+  (enum $add_type_sid:path)    $add_sid:ident    )*
                $(&  (enum $bitand_type_sid:path) $bitand_sid:ident )*
                $(^  (enum $bitxor_type_sid:path) $bitxor_sid:ident )*
                $(/  (enum $div_type_sid:path)    $div_sid:ident    )*
                $(*  (enum $mul_type_sid:path)    $mul_sid:ident    )*
                $(<< (enum $shl_type_sid:path)    $shl_sid:ident    )*
                $(>> (enum $shr_type_sid:path)    $shr_sid:ident    )*
                ,$(#[$comment:meta])*
            )+
        }
    )+
    ) => {
        $(
            $(#[$outer_comment])?
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
            $outer_vis struct $name(isize);
            __encap_enum_impl!{$name, isize}
            impl $name {
                #[allow(dead_code)]
                pub fn iter() -> core::slice::Iter<'static, isize> {

                    const _ARRAY: &[isize] = &[$($name :: $val_name .0,)+];
                    _ARRAY.into_iter()
                }

                $(
                    $(#[$comment])*
                    #[allow(non_upper_case_globals)]
                    pub const $val_name: $name = $name (
                        $($li $(- $sub_li)* $(- $name :: $li_sub_id .0)* $(- $type0 ($li_sub_sid) .0 )*)?
                        $($name :: $id .0 $(- $name :: $sub_id .0)* $(- $id_sub_li)* $(- $type1 ($id_sub_sid) .0 )*)?
                        $($type2 ($($sid0)? $(- $sid1)*) .0 $(- $($sid_sub_li)* )*)*                    
                        $(- $name :: $id1 .0)?

                        $(|  $bitor_li  )*
                        $(+  $add_li    )*
                        $(&  $bitand_li )*
                        $(^  $bitxor_li )*
                        $(/  $div_li    )*
                        $(*  $mul_li    )*
                        $(<< $shl_li    )*
                        $(>> $shr_li    )*

                        $(|  $name :: $bitor_id .0  )*
                        $(+  $name :: $add_id .0    )*
                        $(&  $name :: $bitand_id .0 )*
                        $(^  $name :: $bitxor_id .0 )*
                        $(/  $name :: $div_id .0    )*
                        $(*  $name :: $mul_id .0    )*
                        $(<< $name :: $shl_id .0    )*
                        $(>> $name :: $shr_id .0    )*

                        $(|  $bitor_type_sid ($bitor_sid)   .0 )*
                        $(+  $add_type_sid ($add_sid)       .0 )*
                        $(&  $bitand_type_sid ($bitand_sid) .0 )*
                        $(^  $bitxor_type_sid($bitxor_sid)  .0 )*
                        $(/  $div_type_sid ($div_sid)       .0 )*
                        $(*  $mul_type_sid ($mul_sid)       .0 )*
                        $(<< $shl_type_sid ($shl_sid)       .0 )*
                        $(>> $shr_type_sid ($shr_sid)       .0 )*
                    );
                )+
            }
        )+
    };
    (
        $(#[$outermost_comment:meta])*
        $whole_vis:vis mod $namespace:ident {
            $(
                $(#[$outer_comment:meta])*
                $outer_vis:vis enum $name:ident {
                    $($val_name:ident,$(#[$comment:meta])*)+
                }
            )+
        }
    ) => {
        $(#[$outermost_comment])*
        #[allow(non_camel_case_types)]
        $whole_vis mod $namespace {
            mod __encap_enum {
                $(
                    #[allow(non_snake_case)]
                    pub mod $name {
                        pub enum $namespace {
                            $($val_name,)+            
                        }
                    }
                )+
            }
            $(
                $(#[$outer_comment])*
                #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
                $outer_vis struct $name(isize);
                __encap_enum_impl!{$name, isize}            
                
                impl $name {                    
                    pub fn iter() -> core::slice::Iter<'static, isize> {
                        const _ARRAY: &[isize] = &[$($name :: $val_name .0,)+];
                        _ARRAY.into_iter()
                    }    


                    $(
                        $(#[$comment])*
                        #[allow(non_upper_case_globals)]
                        pub const $val_name: $name = $name ( __encap_enum :: $name:: $namespace :: $val_name as isize);
                    )+
                }
            )+
        }
    };    
    (
        $(#[$outermost_comment:meta])*
        $whole_vis:vis mod $namespace:ident {
            $(
                $(#[$outer_comment:meta])*
                $outer_vis:vis enum $name:ident : $inner_vis:vis $type:ty{
                    $($val_name:ident,$(#[$comment:meta])*)+
                }
            )+
        }
    ) => {
        $(#[$outermost_comment])*
        #[allow(non_camel_case_types)]
        $whole_vis mod $namespace {
            mod __encap_enum {
                $(
                    #[allow(non_snake_case)]
                    pub mod $name {
                        pub enum $namespace {
                            $($val_name,)+            
                        }
                    }
                )+
            }
            $(
                $(#[$outer_comment])*
                #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
                $outer_vis struct $name($inner_vis $type);
                __encap_enum_impl!{$name, $type}            
                
                impl $name {

                    pub fn iter() -> core::slice::Iter<'static, $type> {
                        const _ARRAY: &[$type] = &[$($name :: $val_name .0,)+];
                        _ARRAY.into_iter()
                    }
                    
                    $inner_vis fn get_bit(&self, bit:u8) -> bool{

                        self.0 & (1 << bit) != 0
                    }

                    $(
                        $(#[$comment])*
                        #[allow(non_upper_case_globals)]
                        pub const $val_name: $name = $name ( __encap_enum :: $name:: $namespace :: $val_name as $type);
                    )+
                }
            )+
        }
    };
    (        
        $(
            $(#[$outer_comment:meta])*
            $outer_vis:vis enum $name:ident : $inner_vis:vis $type:ty{
                $($val_name:ident,$(#[$comment:meta])*)+
            }
        )+
    ) => { compile_error!("encap_enum requires fieldless enums to be under a 'mod'.")}
}




#[cfg(test)]
mod tests {
    encap_enum!(
        pub mod flag {
            pub enum TestEnum: pub isize {
                Array,
                Bar,
                Jar,
                Foo,
            }
        }
    );

    encap_enum!{
        mod m0 {
            enum MultEnum0 {
                A,
                B,
                C,
            }
        }
    }

    encap_enum!{
        mod m1 {
            enum MultEnum0: u32 {
                A,
                B,
                C,
            }
            enum m1: u32 {
                A,
                B,
                C,
            }
        }
    }

    encap_enum!{
        enum OtherEnum{
            Foo = 0,
            Bar = 1,
            Rust = 2,
        }
        enum DuplicateEnum{
            Foo = 0,
            Bar = 1,
            Rust = 2,
        }
    }

    encap_enum!{
        enum SpecificEnum: pub u32{
            Foo = 0,
            Bar = 1,
            Rust = 2,
        }  
    }

    encap_enum!{
        enum SignedEnum: pub i32 {
            Food = 1 - 3,
            Bard = (-Food) * 3,
            Jard = -3,
            Read = Jard - Bard,
            Yard = (enum SignedEnum) TEST_CONST - (enum SignedEnum) TEST_CONST_TWO,
        }
    }

    encap_enum! {
        mod test_flag {
            enum test_flag: u32 {
                A, B, C,
            }
            enum thing2: u32 {
                G, H, J, K,
            }
        }
    }

    const TEST_CONST: i32 = 64;
    const TEST_CONST_TWO: i32 = 57;

    // Verify the relationship between enum values and raw values.
    #[test]
    fn value_integrity() {
        let var0 = flag::TestEnum::Array;
        let var1 = flag::TestEnum::Bar | flag::TestEnum::Jar;
        assert_eq!(-OtherEnum::Bar.0, -1); // Replacing OtherEnum with SpecificEnum results in an unsigned error, which is expected
        assert_eq!(var0.0, 0);
        assert_eq!(var1.0, 3);
        assert_eq!(SignedEnum::Bard, 6.into());
        assert_eq!(SignedEnum::Yard.0, TEST_CONST - TEST_CONST_TWO);
        assert_eq!(SignedEnum::Bard.get_bit(1), true);
    }

    // Verify the into and from traits work as intended.
    #[test]
    fn into_from() {
        let var: flag::TestEnum = flag::TestEnum::from(54);
        assert_eq!(var, 54.into());
    }

    // Verify the enum can iterate
    #[test]
    fn iteration() {
        encap_enum!{
            pub mod tmp {
            pub enum GreaterEnum: pub u32{
                Bar,
                Foo,                
                Rust,
            }  
            }
        }
        assert_ne!(tmp::GreaterEnum::Bar.0 as isize, flag::TestEnum::Bar.0);

        let mut count = 0isize;
        for t in flag::TestEnum::iter() {
            assert_eq!(&count, t);
            count += 1;
        }

        count = 0;
        for t in OtherEnum::iter() {
            assert_eq!(&count, t);
            count += 1;
        }

        let mut count = 0u32;
        for t in SpecificEnum::iter() {
            assert_eq!(&count, t);
            count += 1;
        }

        count = 0;
        for t in tmp::GreaterEnum::iter() {
            assert_eq!(&count, t);
            count += 1;
        }
    }

    #[test]
    fn enumeration() {
        let mut count = 0;
        #[allow(clippy::explicit_counter_loop)]
        for (t, _y) in flag::TestEnum::iter().enumerate() {
            assert_eq!(count, t);
            count += 1;
        }
    }

    // verify if the assignment ops have been implemented
    #[test]
    fn assignment() {        
        encap_enum!{
            pub mod thing {
                pub enum GreaterEnum: pub u32{
                    Bar,
                    Foo,                
                    Rust,
                }  
            }
        }

        let a = flag::TestEnum::Array;
        let mut b = flag::TestEnum::Bar;
        b += a;
        b -= a;
        b *= a;
        b &= a;
        b |= a;
        b /= 54.into();
        b ^= 6.into();
        b %= 30.into();

        let t = OtherEnum::Rust;
        let mut z = OtherEnum::Foo;
        z += t;
        z -= t;
        z *= t;
        z &= t;
        z |= t;
        z /= t;
        z ^= t;
        z %= t;

        let t = SpecificEnum::Rust;
        let mut z = SpecificEnum::Foo;
        z += t;
        z -= t;
        z *= t;
        z &= t;
        z |= t;
        z /= t;
        z ^= t;
        z %= t;

        let t = thing::GreaterEnum::Rust;
        let mut z = thing::GreaterEnum::Foo;
        z += t;
        z -= t;
        z *= thing::GreaterEnum::Bar;
        z &= t;
        z |= t;
        z /= t;
        z ^= t;
        z %= t;
    }
    #[test]
    fn externvar(){
        const AQUA: u32 = 34;
        const TERA: u32 = 64;
        encap_enum!{
            enum ExternEnum: u32{
                Aqua = (enum ExternEnum)AQUA + (enum ExternEnum)TERA,
            }  
        }        
        assert_eq!(ExternEnum::Aqua.0, AQUA + TERA);
    }
}
