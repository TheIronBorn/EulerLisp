; Solved: 17.12.17

(println "Solution: "
         (- (~> (range 1 100) sum square)
            (~> (range 1 100) (map square) sum)))
