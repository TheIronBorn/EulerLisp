; Solved: 27.2.2018

;; $$
;; \begin{aligned}
;; & \sum_{i=1}^{5} (p - i)! \\
;; \equiv_p& (p - 5)! \cdot (1 + (p - 4) + (p - 4)(p - 3) + ...) \\
;; \equiv_p& (p - 5)! \cdot (1 - 4 + 12 - 24 + 24) \\
;; \equiv_p& (p - 5)! \cdot 9
;; \end{aligned}
;; $$
;;
;; __Wilson's Theorem:__ $(p - 1)! = -1 \mod p$
;;
;; With this, we can rewrite the first equation to
;;
;; $$
;; \begin{aligned}
;; & \sum_{i=1}^{5} (p - i)! \\
;; \equiv_p& (p - 5)! \cdot 9 \\
;; \equiv_p & \frac{(p-1)!}{(p-1)(p-2)(p-3)(p-4)} \cdot 9 \\
;; \equiv_p & \frac{-1}{(p-1)(p-2)(p-3)(p-4)} \cdot 9 \\
;; \equiv_p & \frac{1}{24} \cdot -9
;; \equiv_p & 24^{-1} \cdot -9
;; \end{aligned}
;; $$
;;
;; The inverse can be found using Eulers Theorem, $a^{\varphi(n)} \equiv_n 1$
;; if $n$ and $a$ are coprime
;; or with an adapted form of the extended Euclidian Algorithm (`modular-inverse`).

(defn s (p)
  (% (* (+ p (% -9 p))
        (modular-inverse 24 p))
     p))

(~> 100_000_000
    dec
    primes~
    (select~ &(>= &1 5))
    (map~ s)
    sum~
    solution)
