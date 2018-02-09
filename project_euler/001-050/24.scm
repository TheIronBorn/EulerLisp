; Solved 1.1.2018

(defn permutation (index candidates)
      (if (not (empty? candidates))
          (let* ([len (length candidates)]
                 [f (fac (dec len))]
                 [first (div index f)])
            (cons (nth first candidates)
                  (permutation
                    (% index f)
                    (delete-nth first candidates))))))

; Permutations are 0-indexed,
; `permutations~` won't work here because it is not ordered the right way
(~>
  (range 0 9)
  (permutation 999999)
  (apply str)
  solution)
