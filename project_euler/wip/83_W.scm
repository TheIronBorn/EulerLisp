; Solved: 9.2.2018
; Time: 2:16

(def nodes
  (~>
    (file-read "./project_euler/input-files/82.txt")
    lines
    (reject empty?)
    (map &(map string->number (string-split "," &1)))
    ))

(defn neighbors (pos)
  (~>
    (list 
      (pair {(fst pos) + 1} (rst pos))
      (pair {(fst pos) - 1} (rst pos))
      (pair (fst pos) {(rst pos) + 1})
      (pair (fst pos) {(rst pos) - 1}))
    (select
      &(and
         {-1 < (fst &1) < (length nodes)}
         {-1 < (rst &1) < (length (fst nodes))}))))

(def q (list (pair 0 0)))

(def dist
  (filled-list
    {(length nodes) * (length (fst nodes))}
    99999999))

(defn index (pos) {{(fst pos) * (length (fst nodes))} + (rst pos)})
(set-nth! dist (index (pair 0 0)) 0)

(defn lookup-node (pos)
  (nth (rst pos) (list-ref nodes (fst pos))))

(defn insert-if-new (n)
  (unless (any? &(= &1 n) q)
          (push! q n)))

(defn astar (start)
  (if {(length q) > 0}
    (let ([current (min-by &(list-ref dist (index &1)) q)])
      (set! q (delete current q))
      (let ([ns (neighbors current)])
        (each (fn (n)
                (let ([alt
                       {(list-ref dist (index current)) + (lookup-node n) }
                       ])
                  (when {alt < (list-ref dist (index n))}
                    (insert-if-new n)
                    (set-nth! dist (index n) alt))))
              ns))
      (astar start))))

(astar (pair 0 0))
(solution
  {(list-ref dist (index (pair 79 79))) + (lookup-node (pair 0 0))})
