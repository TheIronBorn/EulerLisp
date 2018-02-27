; Solved: 26.2.2018

;; The generalized __McCarthy function__ is defined as
;;
;; $$
;; M_{m,k,s}(n) =
;; \begin{cases}
;;   n - s & \text{ if } n > m \\
;;   M^2_{m,k,s}(n + k) & \text{ if } 0 \leq n \leq m
;; \end{cases}
;; $$

(defn mccarthy (m k s n)
  (if {n > m}
      {n - s}
      (mccarthy m k s (mccarthy m k s {n + k}))))

;; Looking at some of its values,
;; there is an obvious pattern:
;;
;; $M_{17,7,3} =$
;; `(18 15 16 17 18 15 16 17 18 15 16 17 18 15 16 17 18)`

; (~> (range~ 1 17)
;     (map~ &(mccarthy 17 7 3 &1))
;     collect
;     println)

;; [“Textbook Examples of Recursion”](https://arxiv.org/pdf/cs/9301113.pdf)
;; includes a generalization of this function
;; and a proof that for $n \leq m$
;;
;; $$M(n) = m + k - 2s - ((m - n) \mod (k - s))$$
;;
;; What happens if $n + xk \leq m$?
;;
;; $$
;; M^2(n + k) = M^3(n + 2k) = M^4(n + 3k) = \ldots = M^{x+1}(n + xk)
;; $$
;; 
;; until it reaches a point where $n + xk > m$
;; and the result is.
;; Let $c$ be the minimal integer for which $n + ck > m$.
;;
;; $$
;; M^2(n + k) = M^c(n + ck - s)
;; $$
;;
;; Now consider $M(n + k - s)$.
;; Because $1 \leq s < k$, we know that $n < n + k - s < n + k$.
;; So $n + (c-1)k - s < n + (c-1)k \leq m$, by definition of $c$
;;
;; $$
;; M(n + k - s) = M^{1+(c-1)}(n + k - s + (c-1)k) = M^c(n + ck - s)
;; $$
;;
;; This shows that $M(n) = M^2(n + k) = M(n + k - s)$ for $n \leq m$
;; and we can rewrite the definition of $M$ to
;;
;; $$
;; M_{m,k,s}(n) =
;; \begin{cases}
;;   n - s & \text{ if } n > m \\
;;   M_{m,k,s}(n + k - s) & \text{ if } 0 \leq n \leq m
;; \end{cases}
;; $$
;;
;; Now let $n$ be of the form $m - c(k - s) + a$
;; with $c \geq 1$ and $0 < a \leq (k-s)$
;; (c is no the same as before).
;;
;; $n \leq m$ is always true,
;; because $c(k-s) \geq k-s \geq a$.
;;
;; By induction
;; $$
;; \begin{aligned}
;; & M(m - c(k - s) + a)
;; \\ = & M(m - (c-1)(k - s) + a)
;; \\ = & \ldots
;; \\ = & M(m + a)
;; \\ = & m + a - s
;; \end{aligned}
;; $$
;;
;; The last step is to bring any $n \leq m$ into this form.
;; $$
;; n
;; = m - (m - n)
;; = m - ((m - n) % (k - s))
;;
;; M(n)
;; = m - ((m - n) % (k - s)) - s
;; $$
;;
;; TODO: Complete proof, the equation above seems to be wrong


(def p 1_000_000)

(defn solve (p m)
  (~> (range~ 1 (dec p))
      (map~
        (fn (s)
          (~> (range~ 1 (min s {p - s}))
              (select~ &(divides? &1 s))
              (map~ (fn (k)
                      (+ {{m - s} * k} (gauss-sum k))))
              sum~)))
      sum~))

;; Pretty much the same code as above,
;; just iterating over multiples instead of divisors
; (~>
;   (range~ 1 p)
;   (map~
;     (fn (n)
;       (~>
;         (range~ 1 (div p n))
;         (map~ &(* &1 n))
;         ; Here, &1 corresponds to s
;         ; the if is needed b/c
;         ; the second sum goes to min(s, p-s)
;         ; TODO: move the limit to the range?
;         (map~ &(if {n <= {p - &1}}
;                     (+ {n * {p - &1}} (gauss-sum n))
;                     0))
;         sum~)))
;   sum~
;   solution)
(~>
  (range~ 1 p)
  (map~
    (fn (n)
      (~>
        (range~ 1 (dec (div p n)))
        (map~ &(* &1 n))
        ; Here, &1 corresponds to s
        ; the if is needed b/c
        ; the second sum goes to min(s, p-s)
        ; TODO: move the limit to the range?
        (map~ &(+ {n * {p - &1}} (gauss-sum n)))
        sum~)))
  sum~
  solution)

; (solution (solve2 p p))
; (solution res)
