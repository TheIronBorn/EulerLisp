; Solved: 29.12.2017

;; $n = p_1^{e_1} \cdots p_k^{e_k}$
;; has $(e_1 + 1) \cdots (e_k + 1)$ factors,
;; since each prime $p_i$ can be included
;; $0$ to $e_i$ times in the factor.

(defn num-factors (n)
  (~> n
      prime-factors
      (reduce-product &(inc (rst &1)))))

(~> (step-stream 1 1)
    (stream-map gauss-sum)
    (stream-select &(~> &1 num-factors (< 500)))
    fst
    solution)
