; Solved: 12.2.2018

; Placing a 0 in front of and at the end of each row
; makes it possible to use the same algorithm as before
; to find paths that start and end at any y position
(def nodes
  (~>
    "./project_euler/input-files/82.txt"
    input-file-lines
    (map &(list->vector
            (cons 0 (append (map string->number (string-split "," &1)) (list 0)))))
    list->vector))

(defn neighbors (pos)
  (~>
    (list 
      (cons {(fst pos) + 1} (rst pos))
      (cons {(fst pos) - 1} (rst pos))
      (cons (fst pos) {(rst pos) + 1}))
    (select
      &(and
         {-1 < (fst &1) < 80}
         {-1 < (rst &1) < 82}))))

(def q (list (cons 0 0)))
(def dist (make-vector {80 * 82} 99999999))

(defn index (pos) {{(fst pos) * 82} + (rst pos)})
(vector-set! dist (index (cons 0 0)) 0)

(defn lookup-node (pos)
  (vector-ref (vector-ref nodes (fst pos)) (rst pos)))

(defn insert-if-new (n)
  (unless (any? &(= &1 n) q)
          (let ([next (cons n q)])
            (set! q next))))

(defn dijkstra (start)
  (if {(length q) > 0}
    (let ([current (min-by &(vector-ref dist (index &1)) q)])
      (set! q (delete current q))
      (let ([ns (neighbors current)])
        (each (fn (n)
                (let ([alt
                       {(vector-ref dist (index current)) + (lookup-node n) }
                       ])
                  (when {alt < (vector-ref dist (index n))}
                    (insert-if-new n)
                    (vector-set! dist (index n) alt))))
              ns))
      (dijkstra start))))

(dijkstra (cons 0 0))
(solution
  (vector-ref dist (index (cons 79 81))))
