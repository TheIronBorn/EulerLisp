; Solved: 2.3.2018

;; We are looking for the shortest route from $(0, 0, 0)$ to $(a, b, c)$.
;; 
;; There are six possible routes,
;;
;; 1. $(0, 0, 0) \to (x, b, 0) \to (a, b, c)$
;; 2. $(0, 0, 0) \to (a, x, 0) \to (a, b, c)$
;; 3. $(0, 0, 0) \to (x, 0, c) \to (a, b, c)$
;; 4. $(0, 0, 0) \to (a, 0, x) \to (a, b, c)$
;; 5. $(0, 0, 0) \to (0, x, c) \to (a, b, c)$
;; 6. $(0, 0, 0) \to (0, b, x) \to (a, b, c)$
;;
;; The lengths of __1__ and __3__ are
;; $\sqrt{x^2 + b^2} + \sqrt{(a - x)^2 + c^2}$
;; and
;; $\sqrt{(a-x)^2 + b^2} + \sqrt{x^2 + c^2}$.
;;
;; If $x$ is the minimal solution for __1__,
;; $a-x$ will be the minimal solution for __3__
;; and the lengths will be the same.
;; This reduces the relevant cases from six to three.
;;
;; Now we need to find $x$ so that the length is minimal.
;;
;; Let $f(x) = \sqrt{c^2 + x^2} + \sqrt{b^2 + (a - x)^2}$,
;; then
;;
;; $$f'(x) = \frac{x}{\sqrt{c^2 + x^2}} - \frac{a - x}{\sqrt{b^2 + (a - x)^2}}$$
;;
;; and 
;;
;; $$f''(x) = \frac{x}{\sqrt{c^2 + x^2}} - \frac{a - x}{\sqrt{b^2 + (a - x)^2}}$$
;;
;; We are looking for some $x$ so that $f'(x) = 0$
;;
;; $$
;; \begin{aligned}
;; & f'(x) = 0 \\
;; \iff & \frac{x}{\sqrt{c^2 + x^2}} = \frac{a - x}{\sqrt{b^2 + (a - x)^2}} \\
;; \iff & \frac{x}{a - x} = \frac{\sqrt{c^2 + x^2}}{\sqrt{b^2 + (a - x)^2}} \\
;; \iff & \frac{x^2}{(a - x)^2} = \frac{c^2 + x^2}{b^2 + (a - x)^2} \\
;; \iff & x^2(b^2 + (a - x)^2) = (c^2 + x^2)(a - x)^2 \\
;; \iff & x^2b^2 + x^2(a - x)^2 = c^2(a-x)^2 + x^2(a - x)^2 \\
;; \iff & x^2b^2 = c^2(a-x)^2 \\
;; \iff & x^2b^2 = c^2a^2 - 2c^2ax + c^2x^2 \\
;; \iff & (c^2 - b^2)x^2 - 2c^2ax + c^2a^2 = 0 \\
;; \end{aligned}
;; $$
;;
;; In the special case $c = b$, which will become important later
;; $$(c^2 - b^2)x^2 - 2c^2ax + c^2a^2 = 0 \iff x = \frac{a}{2}$$
;;
;; For the remaining cases $x$ can be found using the quadratic formula.
;;
;; $$
;; \begin{aligned}
;; A = & (c^2 - b^2) \\
;; B = & -2ac^2 \\
;; C = & c^2a^2 \\ \\
;; B^2 - 4AC = & 4a^2c^4 - 4(c^2 - b^2)c^2a^2
;; \\ = & 4a^2c^4 - 4a^2c^4 + 4a^2b^2c^2
;; \\ = & (2abc)^2 \\ \\
;; x_1, x_2 = & \frac{-B \pm \sqrt{B^2 - 4AC}}{2A}
;; \\ = & \frac{2ac^2 \pm 2abc}{2c^2 - 2b^2}
;; \\ = & \frac{ac^2 \pm abc}{c^2 - b^2}
;; \end{aligned}
;; $$
;;
;; 
;;
;; $$
;; \begin{aligned}
;; f(x_1)
;; = &
;;   \sqrt{c^2 + \left(\frac{ac^2 - abc}{c^2 - b^2}\right)^2}
;; + \sqrt{b^2 + \left(a - \frac{ac^2 - abc}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c - b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \left(\frac{ac^2 - ab^2 - ac^2 + abc}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c - b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \left(\frac{abc - ab^2}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c-b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c-b)^2}{(c^2 - b^2)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c-b)^2(c + b)^2}{(c^2 - b^2)^2(c + b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c-b)^2(c + b)^2}{(c^2 - b^2)^2(c + b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2((c - b)(c + b))^2}{(c^2 - b^2)^2(c + b)^2)}}
;; + \sqrt{b^2 + \frac{a^2b^2((c - b)(c + b))^2}{(c^2 - b^2)^2(c + b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c^2 - b^2)^2}{(c^2 - b^2)^2(c+b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c^2 - b^2)^2}{(c^2 - b^2)^2(c+b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2}{(c+b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2}{(c+b)^2}}
;; \\ = &
;;   |c|\sqrt{1 + \frac{a^2}{(c+b)^2}}
;; + |b|\sqrt{1 + \frac{a^2}{(c+b)^2}}
;; \\ = &
;;   (|b| + |c|)\sqrt{1 + \frac{a^2}{(c+b)^2}}
;; \\ = &
;;   \sqrt{(c + b)^2 + a^2}
;; && \text{(Because $c, b \geq 0$)}
;; \end{aligned}
;; $$

;; $$
;; \begin{aligned}
;; f(x_2)
;; = &
;;   \sqrt{c^2 + \left(\frac{ac^2 + abc}{c^2 - b^2}\right)^2}
;; + \sqrt{b^2 + \left(a - \frac{ac^2 + abc}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c + b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \left(\frac{ac^2 - ab^2 - ac^2 - abc}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c + b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \left(-\frac{abc + ab^2}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c + b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \left(\frac{abc + ab^2}{c^2 - b^2}\right)^2}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c + b)^2}{(c^2 - b^2)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c + b)^2}{(c^2 - b^2)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c + b)^2(c - b)^2}{(c^2 - b^2)^2(c - b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c + b)^2(c - b)^2}{(c^2 - b^2)^2(c - b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2((c + b)(c - b))^2}{(c^2 - b^2)^2(c - b)^2)}}
;; + \sqrt{b^2 + \frac{a^2b^2((c + b)(c - b))^2}{(c^2 - b^2)^2(c - b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2(c^2 - b^2)^2}{(c^2 - b^2)^2(c - b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2(c^2 - b^2)^2}{(c^2 - b^2)^2(c - b)^2}}
;; \\ = &
;;   \sqrt{c^2 + \frac{a^2c^2}{(c - b)^2}}
;; + \sqrt{b^2 + \frac{a^2b^2}{(c - b)^2}}
;; \\ = &
;;   |c|\sqrt{1 + \frac{a^2}{(c - b)^2}}
;; + |b|\sqrt{1 + \frac{a^2}{(c - b)^2}}
;; \\ = &
;;   (|b| + |c|)\sqrt{1 + \frac{a^2}{(c - b)^2}}
;; \\ = &
;;   (b + c)\sqrt{1 + \frac{a^2}{(c - b)^2}}
;; && \text{(Because $c, b \geq 0$)}
;; \end{aligned}
;; $$
;;
;; Because for $b, c \geq 0$ $(c - b)^2 \leq (c + b)^2$,
;; $f(x_1)$ is always the minimal distance.
;;
;; The lengths of the three possible minimal paths are
;;
;; 1. $\sqrt{(b + c)^2 + a^2}$
;; 2. $\sqrt{(a + c)^2 + b^2}$
;; 3. $\sqrt{(a + b)^2 + c^2}$
;; 
;; $\sqrt(x)$ is strictly monotonically increasing,
;; so in finding the minimal solution, the following forms are equivalent:
;;
;; 1. $b^2 + 2cb + c^2 + a^2$
;; 2. $a^2 + 2ac + c^2 + b^2$
;; 3. $a^2 + 2ab + b^2 + c^2$
;;
;; and
;;
;; 1. $2cb$
;; 2. $2ac$
;; 3. $2ab$
;;
;; Now it is obvious that we get the minimal path
;; by assigning the largest value to $a$
;; and the remaining two to $b$ and $c$.
;;
;; With the condition $a \leq b \leq c$
;; to avoid counting rotations multiple times,
;; this gives us $\sqrt{(a + b)^2 + c^2}$
;; as the distance function.
;;
;; For $M = 1$,
;; there is one assignment of $(a, b, c)$: $(1, 1, 1)$.
;;
;; For $M = 2$,
;; there are four: $(1, 1, 1)$, $(1, 1, 2)$, $(1, 2, 2)$, $(2, 2, 2)$.
;;
;; For $M-1 \to M$, the new assignments are
;; $(a, b, M)$ with $1 \leq a \leq b \leq M$.
;;
;; Considering only $d = a + b$, this reduces to the cases
;; $(d, M)$ with $2 \leq d \leq 2M$.
;;
;; To get the correct number of integral solutions,
;; we need to multiply by the number of ways
;; $d = a + b$ can be written
;; with $1 \leq a \leq b \leq M$.
;; This reduces the runtime for each $M$ from quadratic to linear.
;;
;; __Example__, $M = 5$
;;
;; * $2 = 1 + 1$
;; * $3 = 1 + 2$
;; * $4 = 1 + 3 = 2 + 2$
;; * $5 = 1 + 4 = 2 + 3$
;; * $6 = 1 + 5 = 2 + 4 = 3 + 3$
;; * $7 = 2 + 5 = 3 + 4$
;; * $8 = 3 + 5 = 4 + 4$
;; * $9 = 4 + 5$
;; * $10 = 5 + 5$
;;
;; If $d \leq (M+1)$, the number of solutions is $\left\lfloor \frac{d}{2} \right\rfloor$,
;; otherwise it is $\left\lfloor \frac{2(M+1) - d}{2} \right\rfloor$.

(defn integer-solution? (d a)
  (square? {(square d) + (square a)}))

(defn number-of-assignments (d m)
  (if {d <= (inc m)}
      (div d 2)
      (div {{2 * (inc m)} - d} 2)))

(defn new-integer-solutions (m)
  (~> (range~ 2 {2 * m})
      (map~ &(if (integer-solution? &1 m)
                 (number-of-assignments &1 m)
                 0))
      sum~))

(defn solve ()
  (defn inner (m acc)
    (let ([n {acc + (new-integer-solutions m)}])
         (if {n > 1_000_000}
             m
             (inner (inc m) n))))
  (inner 1 0))

(solution (solve))
