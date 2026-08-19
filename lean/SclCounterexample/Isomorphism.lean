import Mathlib

-- The `decide` calls below reduce free-group identities between words of
-- length up to a few hundred; the defaults are far too tight for that.
set_option maxHeartbeats 8000000
set_option maxRecDepth 4000

set_option relaxedAutoImplicit false
set_option autoImplicit false

/-!
# Isomorphic one-relator groups with relators of different scl

Formal companion to the paper *Isomorphic one-relator groups need not have
relators of equal stable commutator length*, which exhibits

* `r  = aabABabABBAbaabABBAb`  with `scl r  = 1`,
* `r' = aabABabABabABBAbaBAb` with `scl r' = 1/2`,

cyclically reduced words in `F' = [F, F] ≤ F = F(a, b)`, and proves
`⟨a, b | r⟩ ≅ ⟨a, b | r'⟩`. This file formalizes exactly that isomorphism
(Theorem 1 of the paper); it does **not** formalize the scl computation
itself, which is external (Calegari's algorithm, run via `scallop` — see
`scl/` in the repository root).

Let `F = FreeGroup (Fin 2)` play the role of `F(a, b)` and, for `w : F`, let
`⟨a, b | w⟩ := F / ⟨⟨w⟩⟩` be the one-relator group `PresentedGroup {w}`.

The isomorphism is built from Lemma 1 of the paper: given `φ : F →* F` with
`x ↦ p, y ↦ q` and `ψ : F →* F` with `a ↦ s, b ↦ t`, the four memberships

1. `φ r' ∈ ⟨⟨r⟩⟩`   (so `φ` descends to `Φ : G' →* G`)
2. `ψ r  ∈ ⟨⟨r'⟩⟩`  (so `ψ` descends to `Ψ : G →* G'`)
3. `ψ p · x⁻¹, ψ q · y⁻¹ ∈ ⟨⟨r'⟩⟩` (so `Ψ ∘ Φ = id`)
4. `φ s · a⁻¹, φ t · b⁻¹ ∈ ⟨⟨r⟩⟩`  (so `Φ ∘ Ψ = id`)

exhibit `Φ` and `Ψ` as mutually inverse isomorphisms. (Here `x, y` and
`a, b` both name the two generators of `F`; `φ` and `ψ` go between the same
free group, playing the roles of the source and target presentations.)

Each of the six memberships is witnessed by an explicit identity of freely
reduced words in `F`, expressing the word in question as a product of
conjugates of `r^{±1}` (for membership in `⟨⟨r⟩⟩`) or `r'^{±1}` (for
`⟨⟨r'⟩⟩`). Every such identity is a finite computation, checked by `decide`.
-/

namespace SclCounterexample

/-- The free group of rank two. -/
abbrev F := FreeGroup (Fin 2)

/-- Reading a letter of the alphabet `a, A, b, B` as a generator of `F`
(`A = a⁻¹`, `B = b⁻¹`).  Characters other than these four are not used. -/
def letterOf : Char → Fin 2 × Bool
  | 'a' => (0, true)
  | 'A' => (0, false)
  | 'b' => (1, true)
  | _ => (1, false)

/-- Reading a string over the alphabet `a, A, b, B` as an element of `F`. -/
def word (w : String) : F := FreeGroup.mk (w.toList.map letterOf)

/-- The first relator. Must match `\rWord` in `words.tex` at the repository
root (the paper `\input{}`s that file directly, and `scl/relators.py` parses
it; `scripts/check_words.py` checks this literal against it too — embedding
the file here via `include_str` and parsing it inside the kernel made every
`decide` below reduce a `String.Substring`-free but still nontrivial search,
which blew up kernel reduction time from minutes to untenable). -/
def r : F := word "aabABabABBAbaabABBAb"

/-- The second relator. Must match `\rPrimeWord` in `words.tex`; see `r`. -/
def r' : F := word "aabABabABabABBAbaBAb"

/-- The endomorphism `φ` of `F` given by `a ↦ BAbaa`, `b ↦ AABabba`. -/
def phi : F →* F := FreeGroup.lift ![word "BAbaa", word "AABabba"]

/-- The endomorphism `ψ` of `F` given by `a ↦ BabABab`, `b ↦ baBAb`. -/
def psi : F →* F := FreeGroup.lift ![word "BabABab", word "baBAb"]

/-- The one-relator group `⟨a, b | r⟩`. -/
abbrev G : Type := PresentedGroup ({r} : Set F)

/-- The one-relator group `⟨a, b | r'⟩`. -/
abbrev G' : Type := PresentedGroup ({r'} : Set F)

/-! ### Membership in normal closures -/

theorem conj_mem_normalClosure (g x : F) :
    x * g * x⁻¹ ∈ Subgroup.normalClosure ({g} : Set F) := by
  have hg : g ∈ Subgroup.normalClosure ({g} : Set F) :=
    Subgroup.subset_normalClosure (Set.mem_singleton g)
  exact (Subgroup.normalClosure_normal (s := ({g} : Set F))).conj_mem _ hg x

theorem conj_inv_mem_normalClosure (g x : F) :
    x * g⁻¹ * x⁻¹ ∈ Subgroup.normalClosure ({g} : Set F) := by
  have hg : g⁻¹ ∈ Subgroup.normalClosure ({g} : Set F) :=
    inv_mem (Subgroup.subset_normalClosure (Set.mem_singleton g))
  exact (Subgroup.normalClosure_normal (s := ({g} : Set F))).conj_mem _ hg x

/-! ### The six certificates

Each is an identity of freely reduced words in `F`, checked by `decide`. -/

/-- Certificate (1): `φ(r')` is a product of three conjugates of `r^{±1}`. -/
theorem cert_one :
    phi r' =
      (word "baBAABabbaBAbaBAA" * r * (word "baBAABabbaBAbaBAA")⁻¹) *
      (word "ABabbaBAbaBAA" * r * (word "ABabbaBAbaBAA")⁻¹) *
      (word "ABBAb" * r⁻¹ * (word "ABBAb")⁻¹) := by
  decide

/-- Certificate (2): `ψ(r)` is a product of three conjugates of `r'^{±1}`. -/
theorem cert_two :
    psi r =
      (word "BabABaabABabbaBAABabABabbaBABAbaBAA" * r' *
        (word "BabABaabABabbaBAABabABabbaBABAbaBAA")⁻¹) *
      (word "BabABaabABababABabABBAbaBAb" * r'⁻¹ *
        (word "BabABaabABababABabABBAbaBAb")⁻¹) *
      (word "BabAB" * r' * (word "BabAB")⁻¹) := by
  decide

/-- Certificate (3a): `ψ(φ(a)) * a⁻¹` is a conjugate of `r'`. -/
theorem cert_threeA :
    psi (word "BAbaa") * (word "a")⁻¹ =
      word "aBAbaBAA" * r' * (word "aBAbaBAA")⁻¹ := by
  decide

/-- Certificate (3b): `ψ(φ(b)) * b⁻¹` is a conjugate of `r'⁻¹`. -/
theorem cert_threeB :
    psi (word "AABabba") * (word "b")⁻¹ =
      word "ABabABBAbaBAb" * r'⁻¹ * (word "ABabABBAbaBAb")⁻¹ := by
  decide

/-- Certificate (4a): `φ(ψ(a)) * a⁻¹` is a conjugate of `r`. -/
theorem cert_fourA :
    phi (word "BabABab") * (word "a")⁻¹ =
      word "BAABabbaBAbaBAA" * r * (word "BAABabbaBAbaBAA")⁻¹ := by
  decide

/-- Certificate (4b): `φ(ψ(b)) * b⁻¹` is a conjugate of `r⁻¹`. -/
theorem cert_fourB :
    phi (word "baBAb") * (word "b")⁻¹ =
      word "bABBAb" * r⁻¹ * (word "bABBAb")⁻¹ := by
  decide

/-! ### The resulting memberships -/

theorem phi_r'_mem : phi r' ∈ Subgroup.normalClosure ({r} : Set F) := by
  rw [cert_one]
  exact mul_mem (mul_mem (conj_mem_normalClosure _ _) (conj_mem_normalClosure _ _))
    (conj_inv_mem_normalClosure _ _)

theorem psi_r_mem : psi r ∈ Subgroup.normalClosure ({r'} : Set F) := by
  rw [cert_two]
  exact mul_mem (mul_mem (conj_mem_normalClosure _ _) (conj_inv_mem_normalClosure _ _))
    (conj_mem_normalClosure _ _)

theorem threeA_mem :
    psi (word "BAbaa") * (word "a")⁻¹ ∈ Subgroup.normalClosure ({r'} : Set F) := by
  rw [cert_threeA]; exact conj_mem_normalClosure _ _

theorem threeB_mem :
    psi (word "AABabba") * (word "b")⁻¹ ∈ Subgroup.normalClosure ({r'} : Set F) := by
  rw [cert_threeB]; exact conj_inv_mem_normalClosure _ _

theorem fourA_mem :
    phi (word "BabABab") * (word "a")⁻¹ ∈ Subgroup.normalClosure ({r} : Set F) := by
  rw [cert_fourA]; exact conj_mem_normalClosure _ _

theorem fourB_mem :
    phi (word "baBAb") * (word "b")⁻¹ ∈ Subgroup.normalClosure ({r} : Set F) := by
  rw [cert_fourB]; exact conj_inv_mem_normalClosure _ _

/-! ### The two homomorphisms -/

theorem comp_lift {H : Type} [Group H] (f : F →* H) (g : Fin 2 → F) :
    f.comp (FreeGroup.lift g) = FreeGroup.lift fun i => f (g i) := by
  apply FreeGroup.ext_hom
  intro i
  simp

theorem mk_eq_mk_of_mul_inv_mem_normalClosure {rels : Set F} {x y : F}
    (h : x * y⁻¹ ∈ Subgroup.normalClosure rels) :
    PresentedGroup.mk rels x = PresentedGroup.mk rels y :=
  eq_of_mul_inv_eq_one (PresentedGroup.mk_eq_one_iff.mpr h)

/-- The homomorphism `G' →* G` induced by `φ`. -/
def Phi : G' →* G :=
  PresentedGroup.toGroup (rels := ({r'} : Set F))
    (f := fun i => PresentedGroup.mk ({r} : Set F) (![word "BAbaa", word "AABabba"] i))
    (by
      intro x hx
      rw [Set.mem_singleton_iff] at hx
      subst hx
      have h := congrArg (fun (h : F →* G) => h r')
        (comp_lift (PresentedGroup.mk ({r} : Set F)) ![word "BAbaa", word "AABabba"])
      simp only [MonoidHom.comp_apply] at h
      rw [← h]
      exact PresentedGroup.mk_eq_one_iff.mpr phi_r'_mem)

/-- The homomorphism `G →* G'` induced by `ψ`. -/
def Psi : G →* G' :=
  PresentedGroup.toGroup (rels := ({r} : Set F))
    (f := fun i => PresentedGroup.mk ({r'} : Set F) (![word "BabABab", word "baBAb"] i))
    (by
      intro x hx
      rw [Set.mem_singleton_iff] at hx
      subst hx
      have h := congrArg (fun (h : F →* G') => h r)
        (comp_lift (PresentedGroup.mk ({r'} : Set F)) ![word "BabABab", word "baBAb"])
      simp only [MonoidHom.comp_apply] at h
      rw [← h]
      exact PresentedGroup.mk_eq_one_iff.mpr psi_r_mem)

theorem Phi_mk (u : F) :
    Phi (PresentedGroup.mk ({r'} : Set F) u) = PresentedGroup.mk ({r} : Set F) (phi u) := by
  have h := congrArg (fun (h : F →* G) => h u)
    (comp_lift (PresentedGroup.mk ({r} : Set F)) ![word "BAbaa", word "AABabba"])
  simp only [MonoidHom.comp_apply] at h
  exact h.symm

theorem Psi_mk (u : F) :
    Psi (PresentedGroup.mk ({r} : Set F) u) = PresentedGroup.mk ({r'} : Set F) (psi u) := by
  have h := congrArg (fun (h : F →* G') => h u)
    (comp_lift (PresentedGroup.mk ({r'} : Set F)) ![word "BabABab", word "baBAb"])
  simp only [MonoidHom.comp_apply] at h
  exact h.symm

theorem of_eq_mk (i : Fin 2) :
    (PresentedGroup.of i : G) = PresentedGroup.mk ({r} : Set F) (FreeGroup.of i) := rfl

theorem of_eq_mk' (i : Fin 2) :
    (PresentedGroup.of i : G') = PresentedGroup.mk ({r'} : Set F) (FreeGroup.of i) := rfl

theorem Psi_comp_Phi : Psi.comp Phi = MonoidHom.id G' := by
  ext i
  simp only [MonoidHom.comp_apply, MonoidHom.id_apply, of_eq_mk']
  fin_cases i
  · show Psi (Phi (PresentedGroup.mk _ (word "a"))) = _
    rw [Phi_mk, Psi_mk]
    · refine mk_eq_mk_of_mul_inv_mem_normalClosure ?_
      have hp : phi (word "a") = word "BAbaa" := by decide
      rw [hp]
      exact threeA_mem
  · show Psi (Phi (PresentedGroup.mk _ (word "b"))) = _
    rw [Phi_mk, Psi_mk]
    · refine mk_eq_mk_of_mul_inv_mem_normalClosure ?_
      have hq : phi (word "b") = word "AABabba" := by decide
      rw [hq]
      exact threeB_mem

theorem Phi_comp_Psi : Phi.comp Psi = MonoidHom.id G := by
  ext i
  simp only [MonoidHom.comp_apply, MonoidHom.id_apply, of_eq_mk]
  fin_cases i
  · show Phi (Psi (PresentedGroup.mk _ (word "a"))) = _
    rw [Psi_mk, Phi_mk]
    · refine mk_eq_mk_of_mul_inv_mem_normalClosure ?_
      have hs : psi (word "a") = word "BabABab" := by decide
      rw [hs]
      exact fourA_mem
  · show Phi (Psi (PresentedGroup.mk _ (word "b"))) = _
    rw [Psi_mk, Phi_mk]
    · refine mk_eq_mk_of_mul_inv_mem_normalClosure ?_
      have ht : psi (word "b") = word "baBAb" := by decide
      rw [ht]
      exact fourB_mem

/-- **Theorem 1.** The one-relator groups `⟨a, b | r⟩` and `⟨a, b | r'⟩` are
isomorphic, where `r = aabABabABBAbaabABBAb` and `r' = aabABabABabABBAbaBAb`.
(The paper computes `scl r = 1 ≠ 1/2 = scl r'` with `scallop`; see `scl/`.) -/
def groupIso : G ≃* G' :=
  MonoidHom.toMulEquiv Psi Phi Phi_comp_Psi Psi_comp_Phi

/-- `r` and `r'` are different words, so the isomorphism above is not vacuous. -/
theorem relators_ne : r ≠ r' := by decide

theorem groupIso_exists :
    Nonempty (PresentedGroup ({r} : Set F) ≃* PresentedGroup ({r'} : Set F)) :=
  ⟨groupIso⟩

end SclCounterexample
