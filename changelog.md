Change Log
==========
<details>
<<<<<<< Updated upstream
<summary> version 0.1.5 </summary>
=======
<summary> v0.2.1 </summary>

* changed `encap_enum_impl` to `__encap_enum_impl`.
  * `__encap_enum_impl` is hidden from documentation.
* fixed visibility bug for methods.
  * get_bit now has the same visibility as tuple struct data.

</details>
<details>
<summary> v0.2.0 </summary>

* fixed prefix negation bug.
* added `core::ops::Neg`
* **Breaking Changes**: 
  * initialization with local constants outside enum has been changed from `::global_const_name` to C cast syntax, which looks like `(enum ClassStyle) global_const_name`.
  * forced enumerations under a `mod` namespace to get rid of prior limitations.

</details>
<details>
<summary> v0.1.5 </summary>
>>>>>>> Stashed changes

* added ability for external constants to initialize variants

</details>
<details>
<summary> v0.1.4 </summary>

* updated links
* added github badge

</details>
<details>
<summary> v0.1.3 </summary>

* added assignment operators
* updated documentation

</details>
<details>
<summary> v0.1.2 </summary>

* fixed minor bug that uses incorrect fragment in repetition.

</details>
<details>
<summary> v0.1.1 </summary>

* Edited documentation to fix typos

</details>
<details>
<summary> v0.1.0 </summary>

* initial publish to crates.io

</details>