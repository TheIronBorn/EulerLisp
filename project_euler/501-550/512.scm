; Solved: 2.1.2018

; f(n) = sum_{i=1}^n phi(n^i) (mod n+1)
;      = sum_{i=1}^n n^{i-1} phi(n) (mod n+1)
;      = phi(n) sum_{i=1}^n n^{i-1} (mod n+1)
;      = phi(n) sum_{i=0}^{n-1} n^i (mod n+1)
;
; n mod (n + 1) = n
; (n^2) (mod n+1) = ((n+1) - 1)^2 (mod n+1) = 1 (mod n+1)
; => sum_{i=0}^{n-1} n^i (mod n+1) = 1 + n + 1 + n + 1 + n + ...
;   => 0 if n is even
;   => 1 if n is odd,
; 
; So f(n) =
;  n even -> 0
;  n odd -> phi(n)
;
; phi(2m) =
;   2 phi(m) if m even
;   phi(m) if m odd
;
; sum_i^n phi(2x) =
;    sum_i^{n/2} phi(i)
;  + sum_i^{n/2} phi(4x), because phi is added twice for 2m with m even
;    =
;    sum_i^{n/2} phi(i)
;  + sum_i^{n/4} phi(i)
;  + sum_i^{n/4} phi(8x)
;    =
;    ...

(def n 500_000_000)

(defn solve (n acc)
  (if (> n 0)
      (solve (div n 2) (- acc (totient-sum n)))
      acc))

(solution (solve (div n 2) (totient-sum n)))
