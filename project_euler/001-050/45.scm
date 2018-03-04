; Solved: 13.1.2018

(~> (step-stream (inc 285) 1)
    (stream-map triangular)
    (stream-select &(and (pentagonal? &1) (hexagonal? &1)))
    fst
    solution)
