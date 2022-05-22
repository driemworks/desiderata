# Inquirers

An inquiry is an agent in the model, represented as a graph vertex which is an i32, which holds beliefs and is capable of sharing them and communicating with other inquirers in the network. There are two means through which an inquirer can receive new information: inquiry and communication.

In the following, we assume that each inquirer is attempting to determine if, for some condition $p$, if they should believe $p$ or not-$p$.

## Initial Conditions

## Credence
Let $L$ be a classical proposition language. Then we define $\alpha$'s credence in a proposition $p \in L$ at some time t as:

$C_\alpha^t: L \to [0, 1]$


## Inquiry

### Inquiry Results

At any time t, let:

- $S_{i \alpha}^{t+}$ is the proposition: '$\alpha$'s inquiry gives a positive result at time t'
- $S_{i \alpha}^{t-}$ is the proposition: '$\alpha$'s inquiry gives a negative result at time t'

$S_{i \alpha}^t = S_{i \alpha}^{t+} \lor S_{i \alpha}^{t-}$

### Action and Aptitude

An inquirer's action is the probability of receiving any response from inquiry, and their apititude is the propsbility of receiving a positive response given is the 'right' one.

We represent an inquirer's action and aptitude at a time $t$ for some proposition $p \in L$ as the tuple:

$P_{i\alpha}^t = (P(S_{i\alpha}^t), P(S_{i\alpha}^{t+} | S_{i\alpha}^t \land p))$

## Communication

Assume that some inquirer $\beta$ is in communication with $\alpha$. Then we arrive at similar definition for results of communication, action, and aptitude given by:

$S_{\beta \alpha}^t = S_{\beta \alpha}^{t+} \lor S_{\beta \alpha}^{t-}$

and 

$P_{\beta \alpha}^t = (P(S_{\beta \alpha}^t), P(S_{\beta \alpha}^{t+} | S_{\beta \alpha}^t \land p))$

## The network

The basis of our model consists of a digraph.

### Weights

Weights between nodes represents the 'strength' of the link between them, or the chance that a node will send some message to another node. To be more explicity, the weight of the edge from $\beta$ to $\alpha$ is the probability that $\beta$ send a message to $\alpha$, given by $P(S_{\beta \alpha})$.

### Threshold of Assertion

Each link between agents has a 'threshold of assetion' which determines if the agent sends a positive or negative result to another. This is given by a value $T_{\beta \alpha} \in [0, 1]$ such that:

1. If $T_{\beta \alpha} > 0.5$ then $\beta$ send a *positive* message to $\alpha$ if and only if $C_\beta(p) \geq T_{\beta \alpha}$ and a *negative* message if an donly if $C_\beta(p) \leq 1 - T_{\beta \alpha}$
2. If $T_{\beta \alpha} < 0.5$ then $\beta$ send a *positive* message to $\alpha$ if and only if $C_\beta(p) \leq T_{\beta \alpha}$ and a *negative* message if an donly if $C_\beta(p) \geq 1 - T_{\beta \alpha}$
3. If $T_{\beta \alpha} = 0.5$ Then $\beta$ sends any message to $\alpha$ regardless of what she belives by randomly choosing.


## Resources

https://lucris.lub.lu.se/ws/files/5684445/7758617.pdf

https://d1wqtxts1xzle7.cloudfront.net/32111025/overconfidence-libre.pdf?1391486369=&response-content-disposition=attachment%3B+filename%3DTrust_and_the_Value_of_Overconfidence_A.pdf&Expires=1653243513&Signature=PsHlwYzSCH0ubab0QFb5-Sm51PCKO3A1lcVuZczDyritzuGAyVXNsxfuDlmySDvybN6wGGynWfWXBTgs5qZ-D6gdrq6MkQA2gLIoWrm70Vi9IIHr1huzZK7900bZb9zu5czcJiojSzkcKeArqHeE-c0dpUzpYGhuTCMcXk~3y3O~8RuvailTSNiK~2QzLJZ8fNNjTDwAbXrXSTMNRA~aI2zSAxtc22yQo4ep49bofk42GMj4s6Rz2F8HvH6-p-BrhxHw0CYwHMJlxicPPpz7jmD0g4S5CEwuUU138dQsitK-mY6vizxr6HRcLK8YymBIcfQOdCGWQS3JUZ2noAOl0Q__&Key-Pair-Id=APKAJLOHF5GGSLRBV4ZA
