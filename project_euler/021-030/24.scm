; Solved 1.1

(defn permutation (index candidates)
      (if (empty? candidates)
          '()
      (let* ((len (length candidates))
             (f (fac (dec len)))
             (first (/ index f)))
        (cons (nth first candidates)
              (permutation
                (% index f)
                (delete-nth first candidates))))))

; Permutations are 0-indexed
(println "Solution: " (apply str (permutation 999999 (list 0 1 2 3 4 5 6 7 8 9))))
