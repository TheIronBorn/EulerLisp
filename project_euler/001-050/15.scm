; Solved 1.1.2018

(def n 20)

;; Choose $20$ of $40$ positions in the path
;; to move right.
;;
;; $$
;; {40\choose{20}} = \frac{40!}{20!20!}
;; = \frac{\prod_{i = 21}^{40} i}{\prod_{i = 1}^{20} i}
;; = \prod_{i=1}^{20} \frac{20 + i}{i}
;; $$

(solution (product 1 20 &(/ (+ 20 &1) &1)))
