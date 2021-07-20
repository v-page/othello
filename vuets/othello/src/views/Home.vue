<template>
  <div class="home" v-if="inited">
    <!-- <img alt="Vue logo" src="../assets/logo.png" /> -->
    <table class="main">
      <tr class="row" v-for="(row, index_x) in board" :key="index_x">
        <td
          class="column"
          v-for="(cell, index_y) in row"
          :key="index_y"
          @click="reverse(index_x, index_y)"
        >
          <span
            class="cell"
            :class="[is_placeable(index_x, index_y) ? 'placeable' : '']"
          >
            {{ cell }}
          </span>
        </td>
      </tr>
    </table>

    {{ turn }} のターン, 黒: {{ count[0] }} , 白: {{ count[1] }} <br />
    {{ msg }}
  </div>
</template>

<style scoped>
/* .cell{
  font-size: 2ex;

  position:absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
} */

.main {
  width: calc(90vmin - 10px);
}
.column {
  width: calc(90% / 8);
  position: relative;
  font-size: 10vmin;
}
.column:after {
  content: "";
  display: block;
  margin-top: 100%;
}
.column .cell {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(189, 189, 189, 0.637);
}

span.cell.placeable {
  background: rgba(6, 83, 3, 0.637);
}
</style>

<script lang="ts">
let wasm_prom = import("../../../../rust/pkg/");
type wasm = typeof import("../../../../rust/pkg/");

import { Component, Vue } from "vue-property-decorator";
// import HelloWorld from "@/components/HelloWorld.vue"; // @ is an alias to /src

// @Component({
//   components: {
//     // HelloWorld,
//   },
// })

type board = Array<Array<string>>;

@Component
export default class Home extends Vue {
  wasm: wasm | undefined;
  board: board | undefined;
  inited = false;

  readonly player = ["空", "黒", "白"];
  /*
  pub enum Msg {
    UnableToPlace = 0,
    OutOfBoard,
    GameOver,
    Pass,
    NextTurn,
  }
  */
  readonly msg_list = [
    "置けません",
    "内部エラー",
    "ゲーム終了",
    "はパスしました",
    "",
  ];

  msg = "";

  turn: string | undefined;
  count = [0, 0];
  placeable: number[][] | never[] = [];

  init(): void {
    this.wasm?.othello_init();
  }

  is_placeable(x: number, y: number) {
    for (let i = 0; i < this.placeable.length; i++) {
      if (this.placeable[i][0] === x && this.placeable[i][1] === y) {
        return true;
      }
    }
    return false;
  }

  reverse(x: number, y: number) {
    if (this.wasm) {
      let r = this.wasm?.othello_set(x, y);
      if (r === 0 || r === 1) {
        this.msg = this.msg_list[r];
        return;
      } else if (r === 3) {
        let enermy = this.wasm.othello_turn() === 1 ? 2 : 1;
        this.msg = this.player[enermy] + this.msg_list[r];
      } else {
        this.msg = this.msg_list[r];
      }
      this.nextturn();
    }
  }

  nextturn() {
    if (this.wasm) {
      this.parse_board();
      this.turn = this.player[this.wasm.othello_turn()];
      this.count = [
        this.wasm.othello_count_cell(1),
        this.wasm.othello_count_cell(2),
      ];
      this.placeable = this.wasm
        .othello_placeable_list()
        .split("\n")
        .map((x) => x.split("").map((x) => Number(x)));
    }
  }

  parse_board(): void {
    if (this.wasm) {
      this.board = this.wasm
        .othello_board()
        .split("\n")
        .map((x) =>
          x.split("").map((x) => {
            switch (x) {
              case "0":
                return "";

              case "1":
                return "●";

              case "2":
                return "○";

              default:
                return "err";
            }
          })
        );

      // FIXME : forceUpdate
      this.inited = false;
      this.$nextTick(() => (this.inited = true));
    } else {
      throw Error("未初期化");
    }
  }

  async mounted(): Promise<void> {
    this.wasm = await wasm_prom;

    // for debug(普通にやるとeslintがうるさいので)
    eval("window.n = this.wasm");
    this.nextturn();

    this.inited = true;
  }
}
</script>
