# judgefmt

```
usage: judgefmt --name name -l0 p1 ... pk ... -ln p1 ... pk
```

Example:
```
$ judgefmt --name Var -l0 'x : σ ∈ Γ' 'σ ⊑ 𝜏' -l1 'Γ ⊢ₛ x : 𝜏'
     x : σ ∈ Γ    σ ⊑ 𝜏
Var:────────────────────
         Γ ⊢ₛ x : 𝜏
```
