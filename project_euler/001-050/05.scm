; Solved: 17.12.2017

(defn merge (a1 a2)
      (cond
        [(nil? a1) a2]
        [(nil? a2) a1]
        [else (let ([k1 (ffst a1)]
                    [k2 (ffst a2)]
                    [v1 (rfst a1)]
                    [v2 (rfst a2)])
               (cond
                 [(= k1 k2) (cons
                             (cons k1 (max v1 v2))
                             (merge (rst a1) (rst a2)))]
                 [(< k1 k2) (cons
                             (cons k1 v1)
                             (merge (rst a1) a2))]
                 [else (cons
                        (cons k2 v2)
                        (merge a1 (rst a2)))]))]))

(~>
  (range 2 20)
  (map prime-factors)
  (reduce merge '())
  (reduce-product &(pow (fst &1) (rst &1)))
  solution)

