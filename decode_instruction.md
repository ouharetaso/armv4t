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
`opcode[25]`は第3オペランドが即値かそうでないかを判定するのに使ったりする(`I`フラグともいう)

### `opcode[27:25] == 0b000, 0b001` 

データ処理命令  
```
   3                   2                   1
 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|x x x x|0 0 I| opcode|S|  rn   |  rd   |    shifter_operand    |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```
`opcode[25]`はIフラグとして使われる  
`shifter_operand`は`I`フラグによって扱われ方が変わる

#### `opcode[24:21]` opcode

|`opcode`|mnemonic|やること|
|:------:|:------:|-------|
|`0b0000`|`AND`   |bitwise AND|
|`0b0001`|`EOR`   |bitwise Exclusive-OR|
|`0b0010`|`SUB`   |subtract|
|`0b0011`|`RSB`   |reverse subtract|
|`0b0100`|`ADD`   |add|
|`0b0101`|`ADC`   |add with carry|
|`0b0110`|`SBC`   |subtract with carry|
|`0b0111`|`RSC`   |reverse subtract with carry|
|`0b1000`|`TST`   |bitwise AND(only change status register)|
|`0b1001`|`TEQ`   |bitwise Exclusive-OR(only change status register)|
|`0b1010`|`CMP`   |subtract(only change status register)|
|`0b1011`|`CMN`   |add(only change status register)|
|`0b1100`|`ORR`   |bitwise OR|
|`0b1101`|`MOV`   |move|
|`0b1110`|`BIC`   |bit clear|
|`0b1111`|`MVN`   |move negative|








