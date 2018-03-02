; Solved: 3.3.2018

;; Given the prime factorization $n! = \prod_{i=1}^k p_i^{e_i}$,  
;; the unitary divisors are all $\prod_{i \in I} p_i^{e_i}$
;; with an index set $I \subseteq \{1, ..., k\}$.
;;
;;## Factorization of $n!$
;;
;; Take $p_1 = 2$ as an example,
;; in $n!$, $\left\lfloor \frac{n}{2} \right\rfloor$ numbers
;; have at least 2 as a factor,
;; $n!$, $\left\lfloor \frac{n}{2^2} \right\rfloor$ numbers
;; have at least 4 as a factor and so on.
;;
;; So $e_i = \sum_{e = 1}^\infty \left\lfloor \frac{n}{p_i^e} \right\rfloor$
;; which is finite because at some point, all new summands are 0.
;;
;;## Sum of divisors
;;
;; In the case $5! = 2^3 \cdot 3 \cdot 5$,
;; the squares of the unitary divisors are
;;
;; 1. $(2^0)^2 \cdot (3^0)^2 \cdot (5^0)^2$
;; 2. $(2^0)^2 \cdot (3^0)^2 \cdot (5^1)^2$
;; 3. $(2^0)^2 \cdot (3^1)^2 \cdot (5^0)^2$
;; 4. $(2^0)^2 \cdot (3^1)^2 \cdot (5^1)^2$
;; 5. $(2^3)^2 \cdot (3^0)^2 \cdot (5^0)^2$
;; 6. $(2^3)^2 \cdot (3^0)^2 \cdot (5^1)^2$
;; 7. $(2^3)^2 \cdot (3^1)^2 \cdot (5^0)^2$
;; 8. $(2^3)^2 \cdot (3^1)^2 \cdot (5^1)^2$
;;
;; We can write their sum as the sum of
;;
;; 1. $(1 + (2^3)^2) \cdot (3^0)^2 \cdot (5^0)^2$
;; 2. $(1 + (2^3)^2) \cdot (3^0)^2 \cdot (5^1)^2$
;; 3. $(1 + (2^3)^2) \cdot (3^1)^2 \cdot (5^0)^2$
;; 4. $(1 + (2^3)^2) \cdot (3^1)^2 \cdot (5^1)^2$
;;
;; which in turn can be written as the sum of
;;
;; 1. $(1 + (2^3)^2) \cdot (1 + (3^1)^2) \cdot (5^0)^2$
;; 2. $(1 + (2^3)^2) \cdot (1 + (3^1)^2) \cdot (5^1)^2$
;;
;; The pattern should be obvious now,
;; the sum of the squared unitary divisors is equal to
;; $\prod_{i=1}^k (p_i^{2e_i} + 1)$.

(def n 100_000_000)
(def m 1_000_000_009)

(defn count-es (x)
  (defn inner (cur acc)
    (if {cur > n}
        acc
        (inner
          {x * cur}
          {acc + (div n cur)})))
  (inner x 0))

(~> (primes~ n)
    (map~ &(modexp &1 {2 * (count-es &1)} m))
    (map~ inc)
    (reduce~ &(% {&1 * &2} m) 1)
    solution)
