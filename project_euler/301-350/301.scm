;; According to wikipedia,
;;
;; $$
;; X(n_1, n_2, n_3) =
;; \begin{cases}
;;   0 & \text{ if } n_1 \oplus n_2 \oplus n_3 = 0 \\
;;   1 & \text{ otherwise }
;; \end{cases}
;; $$
;;
;; To solve the problem,
;; we need to count all numbers $n$
;;
;; $$
;; \begin{aligned}
;; &
;;   n \oplus 2n \oplus 3n = 0
;; \\ \iff &
;;   n \oplus 2n \oplus (2n + n) = 0
;; \\ \iff &
;;   n \oplus 2n = 2n + n
;; \\ \iff &
;;   n \land 2n = 0
;; \\ \iff &
;;   n \land (n << 1) = 0
;; \end{aligned}
;; $$
;;
;; Some possible $n$ are
;;
;; * $1_2$
;; * $10_2$
;; * $100_2$
;; * $1000_2$
;; * $1010_2$
;; * $101_2$
;; * $1010_2$
;;
;; More formal, the possible $n$ are the ones
;; that don't have at least one $0$ between each two $1$s. 
;;
;; This allows us to define a recursive formula:
;;
;; $S(0) = 0, S(1) = 1$, because there are no solutions for $0$ bits
;; and the only solution for $1$ bit is $1_2$.
;;
;; $$S(n) = 1 + S(n - 1) + S(n - 2)$$
;;
;; For each $n$, $10\ldots0_2$ ($n-1$ zeros) is one possibility,
;; the solutions for $n-2$ prepended with $10_2$ are one,
;; and the last case is the solutions for $n-1$ prepended with $0_2$.
;;
;; The solution is $S(30) + 1$, because is $2^30$ a valid, too.

(defn solve-n-bits (n)
  (defn inner (a b n)
    (if (zero? n)
        a
        (inner b (+ 1 a b) (dec n))))
  (inner 0 1 n))

(solution (inc (solve-n-bits 30)))
