; Solved 1.1

(defn permutation (index candidates)
      (if (empty? candidates)
          '()
      (let* (len (length candidates)
             f (fac (dec len))
             first (div index f))
        (cons (nth first candidates)
              (permutation
                (% index f)
                (delete-nth first candidates))))))

; Permutations are 0-indexed
(~>
  (range 0 9)
  (permutation 999999)
  (apply str)
  (println "Solution: "))
