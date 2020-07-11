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

Conversely, as of `v0.1.5`, constants outside the enum can be used to initialize variants:
```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){ assert_eq!(TakenFlags::Delta.0, 128);}
const VALUE: u32 = 56;
const OTHER: u32 = 72;
encap_enum!{
    enum TakenFlags: u32 {
        Omega = ::VALUE, // Omega = 56
        Sigma = ::OTHER, // Sigma = 72
        Delta = ::VALUE + ::OTHER, // Delta = 128
    }
}
```

The `enum_encap!` macro also supports enumeration like a normal fieldless enum by tapping into rust's own `enum`.
```rust
#[macro_use]
extern crate encap_enum;

encap_enum!{
    enum OtherFlags {
        Alpha,
        Beta,
        Charlie,
    }
}
fn main() {
    println!("{}", OtherFlags::Alpha.0);
}
```
The internal enum is under the non-public namespace `__encap_enum`. Unfortunately, a fieldless enum may only be declared once per scope due to namespace clashes.

## Visibility
The visibility for both the `enum` and variants can be changed:

```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{
    pub enum PublicFlags: pub(in crate) u32 {
        Alpha,
        Beta,
        Charlie,
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
```rust
# #[macro_use]
# extern crate encap_enum;
# fn main(){}
encap_enum!{    
    enum Flag: u32 {
        // A doc comment here would cause errors.
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
#       Hidden,
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
        $(#[$outer_comment:meta])*
        $outer_vis:vis enum $name:ident {
            $(
                $val_name:ident =
                    $(   $li:literal       )?  $(   $id:ident       )?
                    $(|  $bitor_li:literal )*  $(|  $bitor_id:ident )*
                    $(+  $add_li:literal   )*  $(+  $add_id:ident   )*
                    $(&  $bitand_li:literal)*  $(&  $bitand_id:ident)*
                    $(^  $bitxor_li:literal)*  $(^  $bitxor_id:ident)*
                    $(/  $div_li:literal   )*  $(/  $div_id:ident   )*
                    $(*  $mul_li:literal   )*  $(*  $mul_id:ident   )*
                    $(<< $shl_li:literal   )*  $(<< $shl_id:ident   )*
                    $(>> $shr_li:literal   )*  $(>> $shr_id:ident   )*
                    $(-  $sub_li:literal   )*  $(-  $sub_id:ident   )*
                    
                    $(   :: $sid:ident        )*
                    $(|  :: $bitor_sid:ident  )*
                    $(+  :: $add_sid:ident    )*
                    $(&  :: $bitand_sid:ident )*
                    $(^  :: $bitxor_sid:ident )*
                    $(/  :: $div_sid:ident    )*
                    $(*  :: $mul_sid:ident    )*
                    $(<< :: $shl_sid:ident    )*
                    $(>> :: $shr_sid:ident    )*
                    $(-  :: $sub_sid:ident    )*
                ,$(#[$comment:meta])*
            )+
        }
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
            
            $(
                $(#[$comment])*
                #[allow(non_upper_case_globals)]
                pub const $val_name: $name = $name (
                    $(   $li       )?   $(   $name :: $id        .0)?
                    $(|  $bitor_li )*   $(|  $name :: $bitor_id  .0)*
                    $(+  $add_li   )*   $(+  $name :: $add_id    .0)*
                    $(&  $bitand_li)*   $(&  $name :: $bitand_id .0)*
                    $(^  $bitxor_li)*   $(^  $name :: $bitxor_id .0)*
                    $(/  $div_li   )*   $(/  $name :: $div_id    .0)*
                    $(*  $mul_li   )*   $(*  $name :: $mul_id    .0)*
                    $(<< $shl_li   )*   $(<< $name :: $shl_id    .0)*
                    $(>> $shr_li   )*   $(>> $name :: $shr_id    .0)*
                    $(-  $sub_li   )*   $(-  $name :: $sub_id    .0)*
                    
                    $(   $sid        )*
                    $(|  $bitor_sid  )*
                    $(+  $add_sid    )*
                    $(&  $bitand_sid )*
                    $(^  $bitxor_sid )*
                    $(/  $div_sid    )*
                    $(*  $mul_sid    )*
                    $(<< $shl_sid    )*
                    $(>> $shr_sid    )*
                    $(-  $sub_sid    )*
                );
            )+
        }
    };
    (
        $(#[$outer_comment:meta])*
        $outer_vis:vis enum $name:ident {
            $($val_name:ident,$(#[$comment:meta])*)+
        }
    ) => {
        $(#[$outer_comment])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
        $outer_vis struct $name(isize);
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
        impl core::convert::From<isize> for $name {
            fn from(right: isize) -> Self { Self(right as isize) }
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
        mod __encap_enum {
            pub enum $name {
                $($val_name,)+
            }
        }

        impl $name {
            #[allow(dead_code)]
            fn iter() -> core::slice::Iter<'static, isize> {
                const _ARRAY: &[isize] = &[$($name :: $val_name .0,)+];
                _ARRAY.into_iter()
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
        $(#[$outer_comment:meta])*
        $outer_vis:vis enum $name:ident : $inner_vis:vis $type:ty{
            $($val_name:ident,$(#[$comment:meta])*)+
        }
    ) => {
        $(#[$outer_comment])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
        $outer_vis struct $name($inner_vis $type);
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
        impl core::convert::From<$type> for $name {
            fn from(right: $type) -> Self { Self(right as $type) }
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
        mod __encap_enum {
            pub enum $name {
                $($val_name,)+
            }
        }

        impl $name {
            #[allow(dead_code)]
            fn iter() -> core::slice::Iter<'static, $type> {
                const _ARRAY: &[$type] = &[$($name :: $val_name .0,)+];
                _ARRAY.into_iter()
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
}

#[cfg(test)]
mod tests {
    encap_enum!(
        pub enum TestEnum {
            Array,
            Bar,
            Jar,
            Foo,
        }
    );

    encap_enum!{
        enum OtherEnum{
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


    // Verify the relationship between enum values and raw values.
    #[test]
    fn value_integrity() {
        let var0 = TestEnum::Array;
        let var1 = TestEnum::Bar | TestEnum::Jar;

        assert_eq!(var0.0, 0);
        assert_eq!(var1.0, 3);
    }

    // Verify the into and from traits work as intended.
    #[test]
    fn into_from() {
        let var: TestEnum = TestEnum::from(54);
        assert_eq!(var, 54.into());
    }

    // Verify the enum can iterate
    #[test]
    fn iteration() {
        encap_enum!{
            enum GreaterEnum: pub u32{
                Bar,
                Foo,                
                Rust,
            }  
        }
        assert_ne!(GreaterEnum::Bar.0 as isize, TestEnum::Bar.0);

        let mut count = 0isize;
        for t in TestEnum::iter() {
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
        for t in GreaterEnum::iter() {
            assert_eq!(&count, t);
            count += 1;
        }
    }

    #[test]
    fn enumeration() {
        let mut count = 0;
        for (t, _y) in TestEnum::iter().enumerate() {
            assert_eq!(count, t);
            count += 1;
        }
    }

    // verify if the assignment ops have been implemented
    #[test]
    fn assignment() {        
        encap_enum!{
            enum GreaterEnum: pub u32{
                Bar,
                Foo,                
                Rust,
            }  
        }

        let a = TestEnum::Array;
        let mut b = TestEnum::Bar;
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

        let t = GreaterEnum::Rust;
        let mut z = GreaterEnum::Foo;
        z += t;
        z -= t;
        z *= GreaterEnum::Bar;
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
                Aqua = ::AQUA + ::TERA,
            }  
        }
        assert_eq!(ExternEnum::Aqua.0, AQUA + TERA);
    }
}
