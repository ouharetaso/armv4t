# ARMv4T命令のデコード
## `cond`フィールド (`opcode[31:28]`)

ステータスレジスタを見て命令を実行するかを決める
| `cond` |ニーモニックの末尾につけるやつ|意味     |実行する条件|
|:------:|:-----------------------:|--------|:--------:|
|`0b0000`|`EQ`                     |**EQ**ual    | `Z == 1`   |
|`0b0001`|`NE`                     |**N**ot **E**qual| `Z == 0`   |
|`0b0010`|`CS/HS`                  |**C**arry **S**et/unsigned **H**igher or **S**ame| `C == 1` |
|`0b0011`|`CC/LO`                  |**C**arry **C**lear/unsigned **LO**wer| `C == 0`   |
|`0b0100`|`MI`                     |**MI**nus    | `N == 1`   |
|`0b0101`|`PL`                     |**PL**us or zero| `N == 0`   |
|`0b0110`|`VS`                     |Overflow (**V** **S**et)| `V == 1`   |
|`0b0111`|`VC`                     |Not Overflow (**V** **C**lear)| `V == 0` |
|`0b1000`|`HI`                     |unsigned **HI**gher    | `C == 1 && Z == 0`   |
|`0b1001`|`LS`                     |unsigned **L**ower or **S**ame| `C == 0 && Z == 1`   |
|`0b1010`|`GE`                     |signed **G**reater than or **E**qual    | `N == V`   |
|`0b1011`|`LT`                     |signed **L**ess **T**han| `N != V`   |
|`0b1100`|`GT`                     |signed **G**reater **T**han | `Z == 0 && N == V`   |
|`0b1101`|`LE`                     |signed **L**ess than or **E**qual| `Z == 1 && N != V`   |
|`0b1110`|`AL`                     |**AL**ways(常に実行)| - |
|`0b1111`|`(NV)`                     |よくわかんない| - |


## `opcode[27:25]`

おおまかに実行される命令の種類が決まる  
`opcode[25]`はオペランドの一つが即値かそうでないかを判定するのに使ったりする(Iフラグともいう)

### `opcode[27:25] == 0b000, 0b001` 

データ処理命令  
`opcode[25]`はIフラグとして使われる

#### `opcode[24:21]` opcode








